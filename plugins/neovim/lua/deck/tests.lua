local M = {}

local rpc = require("deck.rpc")
local buffers = require("deck.buffers")

local function get_compiler_cmd(file_path, filetype)
  if filetype == "cpp" or filetype == "c" then
    local bin = file_path:gsub("%..*$", "")
    return { "g++", "-std=c++17", "-O2", "-o", bin, file_path }, bin
  elseif filetype == "rust" then
    local bin = file_path:gsub("%.rs$", "")
    return { "rustc", file_path, "-o", bin }, bin
  elseif filetype == "python" then
    return { "python3", file_path }, nil
  else
    return { "echo", "unsupported language" }, nil
  end
end

local function run_one(bin_path, input, timeout_ms)
  timeout_ms = timeout_ms or 5000
  if not bin_path then
    return nil, "No binary"
  end
  local output = vim.fn.system("timeout " .. math.floor(timeout_ms / 1000) .. " " .. vim.fn.shellescape(bin_path), input)
  local exit_code = vim.v.shell_error
  if exit_code == 124 then
    return nil, "TLE"
  elseif exit_code ~= 0 then
    return nil, "RTE"
  end
  -- Trim trailing newlines for comparison
  output = output:gsub("%s+$", "")
  return output, nil
end

function M.run(buf, problem_id)
  problem_id = problem_id or ""
  local file_path = buffers.get_file_path(buf)
  if not file_path then
    vim.notify("Deck: No solution file open", vim.log.levels.ERROR)
    return
  end

  buffers.save_file(buf)
  local filetype = vim.bo[buf].filetype

  -- Fetch test cases via RPC if available
  rpc.send("OpenProblem", {
    problem_id = problem_id,
    platform = "codeforces",
  }, function(resp)
    vim.schedule(function()
      if resp.error then
        vim.notify("Deck: Failed to fetch tests: " .. resp.error.message, vim.log.levels.ERROR)
        return
      end

      local payload = resp.payload
      if type(payload) ~= "table" then
        vim.notify("Deck: Invalid test data", vim.log.levels.ERROR)
        return
      end

      local samples = payload.samples or {}
      if #samples == 0 then
        vim.notify("Deck: No sample tests available", vim.log.levels.WARN)
        return
      end

      -- Compile
      local cmd, bin = get_compiler_cmd(file_path, filetype)
      if bin then
        vim.fn.system(table.concat(vim.tbl_map(vim.fn.shellescape, cmd), " "))
        if vim.v.shell_error ~= 0 then
          vim.notify("Deck: Compilation error", vim.log.levels.ERROR)
          -- Show quickfix with compilation errors
          local compile_output = vim.fn.system(table.concat(vim.tbl_map(vim.fn.shellescape, cmd), " ") .. " 2>&1")
          local qf = {}
          for _, line in ipairs(vim.split(compile_output, "\n")) do
            table.insert(qf, { filename = file_path, text = line, type = "E" })
          end
          vim.fn.setqflist(qf, "r")
          vim.cmd("copen")
          return
        end
      end

      -- Run tests
      local qf = {}
      local passed = 0
      local failed = 0

      for i, sample in ipairs(samples) do
        local expected = (sample.output or ""):gsub("%s+$", "")
        local actual, err = run_one(bin, sample.input or "")

        if err then
          failed = failed + 1
          table.insert(qf, {
            filename = file_path,
            lnum = 1,
            col = 1,
            text = string.format("❌ Test %d: %s", i, err),
            type = "E",
          })
        elseif actual == expected then
          passed = passed + 1
          table.insert(qf, {
            filename = file_path,
            lnum = 1,
            col = 1,
            text = string.format("✅ Test %d: Passed", i),
            type = "I",
          })
        else
          failed = failed + 1
          table.insert(qf, {
            filename = file_path,
            lnum = 1,
            col = 1,
            text = string.format("❌ Test %d: Failed", i),
            type = "E",
          })
          table.insert(qf, {
            filename = file_path,
            lnum = 1,
            col = 1,
            text = "   Input: " .. (sample.input or ""):gsub("\n", " "),
            type = "E",
          })
          table.insert(qf, {
            filename = file_path,
            lnum = 1,
            col = 1,
            text = "   Expected: " .. expected:gsub("\n", " "),
            type = "E",
          })
          table.insert(qf, {
            filename = file_path,
            lnum = 1,
            col = 1,
            text = "   Got: " .. (actual or ""):gsub("\n", " "),
            type = "E",
          })
        end
      end

      table.insert(qf, 1, {
        filename = file_path,
        lnum = 1,
        col = 1,
        text = string.format("=== %d/%d passed ===", passed, #samples),
        type = failed > 0 and "E" or "I",
      })

      vim.fn.setqflist(qf, "r")
      vim.cmd("copen")

      if failed > 0 then
        vim.notify(string.format("Deck: %d/%d tests failed", failed, #samples), vim.log.levels.ERROR)
      else
        vim.notify(string.format("Deck: All %d tests passed", #samples), vim.log.levels.INFO)
      end

      -- Cleanup binary
      if bin then
        vim.fn.delete(bin)
      end
    end)
  end)
end

return M
