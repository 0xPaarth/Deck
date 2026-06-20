local M = {}

M.statement_buf = nil
M.solution_buf = nil
M.statement_win = nil
M.solution_win = nil
M.focus_mode = false

local config = require("deck.config")

function M.create_split(problem_data, solution_file)
  -- Close existing layout
  M.close()

  -- Open solution file (right side)
  vim.cmd("edit " .. vim.fn.fnameescape(solution_file))
  M.solution_buf = vim.api.nvim_get_current_buf()
  M.solution_win = vim.api.nvim_get_current_win()

  -- Create statement split (left side)
  vim.cmd("leftabove vsplit")
  vim.cmd("enew")
  M.statement_buf = vim.api.nvim_get_current_buf()
  M.statement_win = vim.api.nvim_get_current_win()

  -- Resize to configured width
  local width = math.floor(vim.o.columns * config.get().statement_width)
  vim.cmd("vertical resize " .. width)

  -- Configure statement buffer
  vim.bo[M.statement_buf].buftype = "nofile"
  vim.bo[M.statement_buf].modifiable = true
  vim.bo[M.statement_buf].swapfile = false
  vim.bo[M.statement_buf].buflisted = false

  local lines = vim.split(problem_data.statement or "", "\n")
  vim.api.nvim_buf_set_lines(M.statement_buf, 0, -1, false, lines)

  vim.bo[M.statement_buf].modifiable = false
  vim.bo[M.statement_buf].filetype = "markdown"

  vim.wo[M.statement_win].wrap = true
  vim.wo[M.statement_win].number = false
  vim.wo[M.statement_win].relativenumber = false
  vim.wo[M.statement_win].signcolumn = "no"
  vim.wo[M.statement_win].cursorline = true

  -- Rename buffers for identification
  local ok = pcall(vim.api.nvim_buf_set_name, M.statement_buf,
    "deck://statement/" .. problem_data.title)
  if not ok then
    -- name may already exist, ignore
  end

  -- Return focus to solution buffer
  vim.api.nvim_set_current_win(M.solution_win)
end

function M.toggle_focus()
  if not M.statement_win or not vim.api.nvim_win_is_valid(M.statement_win) then
    return
  end

  if M.focus_mode then
    -- Return to split
    local width = math.floor(vim.o.columns * config.get().statement_width)
    vim.api.nvim_set_current_win(M.statement_win)
    vim.cmd("vertical resize " .. width)
    if M.solution_win and vim.api.nvim_win_is_valid(M.solution_win) then
      vim.api.nvim_set_current_win(M.solution_win)
    end
    M.focus_mode = false
  else
    -- Expand statement
    vim.api.nvim_set_current_win(M.statement_win)
    vim.cmd("vertical resize " .. math.floor(vim.o.columns * 0.8))
    vim.cmd("wincmd h")
    if M.solution_win and vim.api.nvim_win_is_valid(M.solution_win) then
      vim.api.nvim_set_current_win(M.solution_win)
    end
    M.focus_mode = true
  end
end

function M.adjust_ratio(direction)
  if not M.statement_win or not vim.api.nvim_win_is_valid(M.statement_win) then
    return
  end
  local step = 5
  vim.api.nvim_set_current_win(M.statement_win)
  if direction == "increase" then
    vim.cmd("vertical resize " .. (vim.api.nvim_win_get_width(M.statement_win) + step))
  elseif direction == "decrease" then
    vim.cmd("vertical resize " .. (vim.api.nvim_win_get_width(M.statement_win) - step))
  end
end

function M.get_solution_buf()
  return M.solution_buf
end

function M.get_solution_path()
  if M.solution_buf and vim.api.nvim_buf_is_valid(M.solution_buf) then
    return vim.api.nvim_buf_get_name(M.solution_buf)
  end
  return nil
end

function M.close()
  -- Wipe statement buffer
  if M.statement_buf and vim.api.nvim_buf_is_valid(M.statement_buf) then
    pcall(vim.api.nvim_buf_delete, M.statement_buf, { force = true })
  end
  M.statement_buf = nil
  M.statement_win = nil
  M.focus_mode = false
end

return M
