local M = {}

local uv = vim.uv or vim.loop
local config = require("deck.config")
local client_pipe = nil
local pending = {}
local buf = ""
local next_id = 1

local function parseBufferedResponses()
  while true do
    local line_end = buf:find("\n")
    if not line_end then
      break
    end
    local line = buf:sub(1, line_end - 1)
    buf = buf:sub(line_end + 1)
    local ok, resp = pcall(vim.json.decode, line)
    if ok and resp.id and pending[resp.id] then
      local cb = pending[resp.id]
      pending[resp.id] = nil
      vim.schedule(function()
        cb(resp)
      end)
    end
  end
end

function M.connect(callback)
  local socket = config.get().socket_path
  if not vim.loop then
    vim.notify("Deck: vim.loop / vim.uv not available", vim.log.levels.ERROR)
    return false
  end

  -- Check if socket exists
  local stat = vim.loop.fs_stat(socket)
  if not stat then
    vim.schedule(function()
      vim.notify(
        "Deck backend not running. Start with: deck-tui",
        vim.log.levels.WARN
      )
      if callback then
        callback(false)
      end
    end)
    return false
  end

  client_pipe = uv.new_pipe(false)
  uv.pipe_connect(client_pipe, socket, function(err)
    if err then
      vim.schedule(function()
        vim.notify(
          "Deck: Failed to connect to " .. socket .. ": " .. err,
          vim.log.levels.ERROR
        )
        if callback then
          callback(false)
        end
      end)
      return
    end

    uv.read_start(client_pipe, function(read_err, data)
      if read_err then
        vim.schedule(function()
          vim.notify("Deck: RPC read error: " .. read_err, vim.log.levels.ERROR)
        end)
        return
      end
      if data then
        buf = buf .. data
        parseBufferedResponses()
      end
    end)

    vim.schedule(function()
      vim.notify("Deck: Connected to backend", vim.log.levels.INFO)
      if callback then
        callback(true)
      end
    end)
  end)

  return true
end

function M.is_connected()
  return client_pipe ~= nil
end

function M.send(req_type, payload, cb)
  if not client_pipe then
    vim.schedule(function()
      vim.notify("Deck: Not connected to backend", vim.log.levels.ERROR)
      if cb then
        cb({ error = { code = 503, message = "Not connected" } })
      end
    end)
    return
  end

  local id = next_id
  next_id = next_id + 1
  pending[id] = cb or function() end

  local req = vim.json.encode({
    id = id,
    type = req_type,
    payload = payload,
  }) .. "\n"

  local ok, write_err = pcall(uv.write, client_pipe, req)
  if not ok then
    pending[id] = nil
    vim.schedule(function()
      vim.notify("Deck: RPC write failed: " .. tostring(write_err), vim.log.levels.ERROR)
      if cb then
        cb({ error = { code = 500, message = "Write failed: " .. tostring(write_err) } })
      end
    end)
  end
end

function M.disconnect()
  if client_pipe then
    uv.close(client_pipe)
    client_pipe = nil
  end
  pending = {}
  buf = ""
end

return M
