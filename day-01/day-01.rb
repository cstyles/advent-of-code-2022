a=STDIN.read.split("\n\n").map{_1.split("\n").map(&:to_i).sum}.sort
p a.last,a.last(3).sum
