if exists('g:loaded_deck')
  finish
endif
let g:loaded_deck = 1

command! -nargs=* DeckOpen lua require('deck').open_problem(<q-args>)
command! DeckRun lua require('deck').run_tests()
command! DeckSubmit lua require('deck').submit()
command! DeckStats lua require('deck').show_stats()
command! DeckTeam lua require('deck').show_team()
command! DeckContest lua require('deck').show_contest()
command! DeckFocus lua require('deck').toggle_focus()
command! DeckNext lua require('deck').next_problem()
command! DeckSync lua require('deck').sync()
command! DeckHelp lua require('deck').show_help()
command! DeckShare lua require('deck').share_solution()
