local M = {}

local config = require("deck.config")
local rpc = require("deck.rpc")
local layout = require("deck.layout")
local buffers = require("deck.buffers")
local tests = require("deck.tests")
local submit = require("deck.submit")
local analytics = require("deck.analytics")
local current_problem_id = nil

function M.setup(opts)
  config.setup(opts or {})
  rpc.connect(function(ok)
    if not ok then
      vim.notify("Deck: RPC connection failed, some features may be unavailable", vim.log.levels.WARN)
    end
  end)
end

function M.open_problem(problem_id)
  problem_id = vim.trim(problem_id or "")
  if problem_id == "" then
    problem_id = vim.fn.input("Problem ID: ", "")
  end
  if problem_id == "" then
    vim.notify("Deck: No problem ID provided", vim.log.levels.WARN)
    return
  end

  current_problem_id = problem_id

  rpc.send("OpenProblem", {
    problem_id = problem_id,
    platform = "codeforces",
  }, function(resp)
    if resp.error then
      vim.schedule(function()
        vim.notify(
          string.format("Deck: [%d] %s", resp.error.code, resp.error.message),
          vim.log.levels.ERROR
        )
      end)
      return
    end

    local data = resp.payload
    if type(data) ~= "table" then
      vim.schedule(function()
        vim.notify("Deck: Invalid response from backend", vim.log.levels.ERROR)
      end)
      return
    end

    -- Build workspace file path
    local title = data.title or problem_id
    local safe_title = title:lower():gsub("[^%w_]", "_"):gsub("_+", "_")
    local workspace = vim.fn.expand("~/.deck/workspace")
    vim.fn.mkdir(workspace, "p")
    local ext = config.get().default_language == "rust" and "rs"
      or (config.get().default_language == "python" and "py" or "cpp")
    local filepath = workspace .. "/" .. problem_id .. "_" .. safe_title .. "." .. ext

    -- Create file if it doesn't exist
    if vim.fn.filereadable(filepath) == 0 then
      local f = io.open(filepath, "w")
      if f then
        f:close()
      end
    end

    vim.schedule(function()
      layout.create_split(data, filepath)
      buffers.write_template(layout.get_solution_buf(), config.get().default_language, problem_id)
      buffers.start_lsp(layout.get_solution_buf(), config.get().default_language)
      M._setup_keymaps(layout.get_solution_buf())
    end)
  end)
end

function M.run_tests()
  tests.run(layout.get_solution_buf(), current_problem_id)
end

function M.submit()
  submit.submit(layout.get_solution_buf(), current_problem_id)
end

function M.show_stats()
  rpc.send("GetStats", { handle = config.get().handle or "" }, function(resp)
    vim.schedule(function()
      analytics.show_stats(resp)
    end)
  end)
end

function M.show_team()
  rpc.send("GetTeamStatus", { team_id = "" }, function(resp)
    vim.schedule(function()
      if resp.error then
        vim.notify("Deck: Team data unavailable", vim.log.levels.INFO)
      else
        analytics.show_floating({ title = "Team Status", content = { "Coming soon..." } })
      end
    end)
  end)
end

function M.show_contest()
  rpc.send("GetContestStatus", { contest_id = "" }, function(resp)
    vim.schedule(function()
      if resp.error then
        vim.notify("Deck: Contest data unavailable", vim.log.levels.INFO)
      else
        analytics.show_floating({ title = "Contest Status", content = { "Coming soon..." } })
      end
    end)
  end)
end

function M.toggle_focus()
  layout.toggle_focus()
end

function M.next_problem()
  vim.notify("Deck: Next recommended problem (coming soon)", vim.log.levels.INFO)
end

function M.sync()
  rpc.disconnect()
  vim.defer_fn(function()
    rpc.connect(function(ok)
      if ok then
        vim.notify("Deck: Synced with backend", vim.log.levels.INFO)
      else
        vim.notify("Deck: Sync failed", vim.log.levels.ERROR)
      end
    end)
  end, 200)
end

function M.share_solution()
  local file_path = buffers.get_file_path(layout.get_solution_buf())
  if not file_path then
    vim.notify("Deck: No solution file to share", vim.log.levels.ERROR)
    return
  end
  buffers.save_file(layout.get_solution_buf())
  rpc.send("ShareSolution", {
    problem_id = current_problem_id or "",
    file_path = file_path,
  }, function(resp)
    vim.schedule(function()
      if resp.error then
        vim.notify("Deck: Share failed: " .. resp.error.message, vim.log.levels.ERROR)
      else
        vim.notify("Deck: Solution shared with team", vim.log.levels.INFO)
      end
    end)
  end)
end

function M.show_help()
  local lines = {
    "Deck — Keyboard Shortcuts",
    "",
    config.get().keymaps.run_tests  .. "  Run tests",
    config.get().keymaps.run_single .. "  Run single test",
    config.get().keymaps.submit     .. "  Submit solution",
    config.get().keymaps.stats      .. "  Show stats",
    config.get().keymaps.team       .. "  Show team",
    config.get().keymaps.contest    .. "  Show contest",
    config.get().keymaps.focus      .. "  Toggle focus mode",
    config.get().keymaps.next       .. "  Next problem",
    config.get().keymaps.sync       .. "  Sync with backend",
    config.get().keymaps.help       .. "  Show this help",
  }
  analytics.show_floating({ title = "Deck Help", content = lines })
end

function M._setup_keymaps(buf)
  local opts = { buffer = buf, silent = true, noremap = true }
  local km = config.get().keymaps

  vim.keymap.set("n", km.run_tests,  function() M.run_tests() end, opts)
  vim.keymap.set("n", km.run_single, function() M.run_tests() end, opts)
  vim.keymap.set("n", km.submit,     function() M.submit() end, opts)
  vim.keymap.set("n", km.stats,      function() M.show_stats() end, opts)
  vim.keymap.set("n", km.team,       function() M.show_team() end, opts)
  vim.keymap.set("n", km.contest,    function() M.show_contest() end, opts)
  vim.keymap.set("n", km.focus,      function() M.toggle_focus() end, opts)
  vim.keymap.set("n", km.next,       function() M.next_problem() end, opts)
  vim.keymap.set("n", km.sync,       function() M.sync() end, opts)
  vim.keymap.set("n", km.help,       function() M.show_help() end, opts)
end

return M
