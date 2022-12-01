a=STDIN.readlines.map(&:to_i).chunk_while{_1!=0}.map(&:sum).sort
p a.last,a.last(3).sum
