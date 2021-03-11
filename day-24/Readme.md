# Day 24

[Advent Of Code Day 24](https://adventofcode.com/2020/day/24).

## Part 1

Flip tiles on the floor.

* tiles are hexagonal
* tiles have two sides, black and white
* tiles start on white side facing up
* there is a list of tiles to flip over
* each line in the list identifies **a single** tile that needs to be flipped, from a reference tile
* every line starts from the same reference tile
* every tile has 6 neighbors (hexagonal)
  * `east` (e), `southeast` (se), `southwest` (sw), `west` (w), `northwest` (nw), `northeast` (ne)
  * a line contains movement instructions, no delimiter
* for example `esenee` means: east, southeast, northeast, east
* from the line, the last tile flips side (black to white or white to black)


## Part 2

Game Of Life mode activated, the hexagonal tile floor is the play grid

* each tile has 6 adjacent tiles
* any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white
* any white tile with exactly two black tiles immediately adjacent to it is flipped to black
* flips are done silmutaneously
