local M = {}

local buffers = require("deck.buffers")
local rpc = require("deck.rpc")

function M.submit(buf, problem_id)
  problem_id = problem_id or ""
  local file_path = buffers.get_file_path(buf)
  if not file_path then
    vim.notify("Deck: No solution file to submit", vim.log.levels.ERROR)
    return
  end

  buffers.save_file(buf)

  local filetype = vim.bo[buf].filetype
  local lang_map = {
    cpp = "cpp",
    c = "c",
    rust = "rust",
    python = "python",
    go = "go",
    java = "java",
  }
  local language = lang_map[filetype] or "cpp"

  rpc.send("Submit", {
    file_path = file_path,
    problem_id = problem_id,
    language = language,
  }, function(resp)
    vim.schedule(function()
      if resp.error then
        vim.notify("Deck: Submit error: " .. resp.error.message, vim.log.levels.ERROR)
        return
      end

      -- Open browser with Codeforces submit page
      local url = string.format(
        "https://codeforces.com/contest/%s/submit",
        problem_id:gsub("%D.*", "")
      )
      local open_cmd = vim.fn.has("mac") == 1 and "open"
        or (vim.fn.has("wsl") == 1 and "wslview" or "xdg-open")
      vim.fn.system(open_cmd .. " " .. vim.fn.shellescape(url))
      vim.notify(
        "Deck: Opening submit page. Fill form and submit manually.",
        vim.log.levels.INFO
      )
    end)
  end)
end

return M
