local M = {}

local config = require("deck.config")

function M.write_template(buf, language, problem_id)
  local templates = {
    cpp = string.format(
      "// %s\n#include <bits/stdc++.h>\nusing namespace std;\n\nint main() {\n    ios::sync_with_stdio(false);\n    cin.tie(nullptr);\n\n    return 0;\n}\n",
      problem_id
    ),
    rust = string.format(
      "// %s\nuse std::io;\n\nfn main() {\n    let mut input = String::new();\n    io::stdin().read_line(&mut input).unwrap();\n}\n",
      problem_id
    ),
    python = string.format(
      "# %s\nimport sys\n\ndef main():\n    data = sys.stdin.read().split()\n\nif __name__ == '__main__':\n    main()\n",
      problem_id
    ),
  }

  local ft_map = {
    cpp = "cpp",
    rust = "rust",
    python = "python",
  }

  vim.bo[buf].modifiable = true
  local template = templates[language] or templates.cpp
  local lines = vim.split(template, "\n", { plain = true })
  vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)
  vim.bo[buf].filetype = ft_map[language] or "cpp"
  vim.bo[buf].modifiable = true
end

function M.start_lsp(buf, language)
  local lsp_cfg = config.get().lsp
  local cmd = lsp_cfg[language] or lsp_cfg.cpp
  if not cmd then
    return
  end

  -- Only attach if not already attached
  local attached = false
  for _, client in ipairs(vim.lsp.get_clients({ bufnr = buf })) do
    if vim.tbl_contains(cmd, client.name) then
      attached = true
      break
    end
  end
  if attached then
    return
  end

  -- Start LSP client
  vim.lsp.start({
    name = "deck_" .. language,
    cmd = cmd,
    root_dir = vim.fn.getcwd(),
  }, {
    bufnr = buf,
    silent = true,
  })
end

function M.save_file(buf)
  if vim.bo[buf].modified then
    vim.api.nvim_buf_call(buf, function()
      vim.cmd("silent write")
    end)
  end
end

function M.get_file_path(buf)
  local name = vim.api.nvim_buf_get_name(buf)
  if name ~= "" then
    return name
  end
  return nil
end

return M
