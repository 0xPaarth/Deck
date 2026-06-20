local M = {}

M.defaults = {
  socket_path = "127.0.0.1:4647",
  default_language = "cpp",
  statement_width = 0.35,
  keymaps = {
    run_tests = "<leader>rt",
    run_single = "<leader>rs",
    submit = "<leader>sb",
    stats = "<leader>sa",
    team = "<leader>st",
    contest = "<leader>sc",
    focus = "<leader>sf",
    next = "<leader>sn",
    sync = "<leader>sg",
    help = "<leader>sh",
  },
  lsp = {
    cpp = { "clangd", "--background-index" },
    rust = { "rust-analyzer" },
    python = { "pyright" },
  },
  render_markdown = true,
  syntax_highlight = true,
}

M.options = {}

function M.setup(opts)
  M.options = vim.tbl_deep_extend("force", M.defaults, opts or {})
end

function M.get()
  return M.options
end

return M
