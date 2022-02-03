function first
  return (string sub --length 1 $argv)
end

set -l _commands cursed random pixelated
set -l commands
for var in _commands
  set -a commands (first $var)
end


complete -c imgutils -n "not __fish_seen_subcommand_from $commands" -a $commands
