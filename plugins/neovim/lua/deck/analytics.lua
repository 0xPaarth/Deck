local M = {}

local rpc = require("deck.rpc")

function M.show_floating(opts)
  opts = opts or {}
  local title = opts.title or "Deck"
  local content = opts.content or {}

  local buf = vim.api.nvim_create_buf(false, true)
  vim.api.nvim_buf_set_lines(buf, 0, -1, false, content)
  vim.bo[buf].modifiable = false
  vim.bo[buf].bufhidden = "wipe"

  local width = math.min(70, vim.o.columns - 4)
  local height = math.min(#content + 2, vim.o.lines - 4)
  local col = math.floor((vim.o.columns - width) / 2)
  local row = math.floor((vim.o.lines - height) / 2)

  local win = vim.api.nvim_open_win(buf, true, {
    relative = "editor",
    width = width,
    height = height,
    col = col,
    row = row,
    style = "minimal",
    border = "rounded",
    title = " " .. title .. " ",
    title_pos = "center",
  })

  vim.keymap.set("n", "q", function()
    if vim.api.nvim_win_is_valid(win) then
      vim.api.nvim_win_close(win, true)
    end
  end, { buffer = buf, silent = true })
  vim.keymap.set("n", "<Esc>", function()
    if vim.api.nvim_win_is_valid(win) then
      vim.api.nvim_win_close(win, true)
    end
  end, { buffer = buf, silent = true })
end

function M.show_stats(resp)
  local lines = {
    "┌──────────────────────────────┐",
    "│         User Stats           │",
    "└──────────────────────────────┘",
    "",
  }

  if resp and resp.payload and type(resp.payload) == "table" then
    local data = resp.payload
    table.insert(lines, "Handle:  " .. tostring(data.handle or "N/A"))
    table.insert(lines, "Rating:  " .. tostring(data.rating or "N/A"))
    table.insert(lines, "Solved:  " .. tostring(data.solved or "N/A"))
    table.insert(lines, "Streak:  " .. tostring(data.streak or "0") .. " days")
    table.insert(lines, "Max:     " .. tostring(data.max_rating or data.rating or "N/A"))
    table.insert(lines, "")

    if data.avg_solve_time then
      local mins = math.floor((data.avg_solve_time or 0) / 60)
      table.insert(lines, "Avg solve time: " .. mins .. " mins")
    end

    if data.predicted_rating then
      table.insert(lines, "")
      table.insert(lines, "┌──────────────────────────────┐")
      table.insert(lines, "│        Predictions           │")
      table.insert(lines, "└──────────────────────────────┘")
      table.insert(lines, "Predicted rating: " .. data.predicted_rating)
      table.insert(lines, "Next milestone:   ~" .. (data.time_to_next_milestone or "?") .. " days")
    end

    if data.weak_tags and #data.weak_tags > 0 then
      table.insert(lines, "")
      table.insert(lines, "┌──────────────────────────────┐")
      table.insert(lines, "│        Weak Tags             │")
      table.insert(lines, "└──────────────────────────────┘")
      for i, wt in ipairs(data.weak_tags) do
        if i > 5 then break end
        local symbol = wt.priority >= 4 and "❌" or "⚠️"
        table.insert(lines, string.format("%s %s (%d%%)", symbol, wt.name, math.floor(wt.accuracy or 0)))
      end
    end

    if data.recent_problems and #data.recent_problems > 0 then
      table.insert(lines, "")
      table.insert(lines, "Recent:")
      for i, rp in ipairs(data.recent_problems) do
        if i > 5 then break end
        table.insert(lines, "  " .. rp)
      end
    end
  else
    lines = {
      "Stats not available yet.",
      "",
      "Mock data:",
      "  Rating: 1600",
      "  Solved: 145",
      "  Streak: 12 days",
      "",
      "Run :DeckSync to refresh.",
    }
  end

  M.show_floating({ title = "Analytics", content = lines })
end

return M
