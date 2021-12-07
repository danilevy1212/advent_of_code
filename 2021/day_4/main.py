#!/usr/bin/env python3
from typing import List, NamedTuple, Set, Tuple

class Cell(NamedTuple):
    mark: bool
    num: int
Board = List[List[Cell]]

def to_board(board: str) -> Board:
    rows = board.splitlines()
    return [[Cell(False, int(elem)) for elem in row.split(' ') if elem.isdecimal()]
            for row in rows]

def find_bingos(boards: List[Board], nums: List[int]) -> List[Tuple[int, Board]]:
    winners: List[Tuple[int, Board]] = []
    winner_idxs: Set[int] = set()

    for num in nums:
        for board_idx, board in enumerate(boards):
            # nop if board already bingo'd
            if board_idx in winner_idxs:
                continue
            # Mutate
            for row_idx, row in enumerate(board):
                for cell_idx, cell in enumerate(row):
                    if (cell.num == num):
                        boards[board_idx][row_idx][cell_idx] = Cell(True, num)
            # Check Bingo
            board_size = len(board)
            for row_idx in range(0, board_size):
                row_bingo = all([boards[board_idx][row_idx][cell_idx].mark
                                for cell_idx in range(0, board_size)])
                col_bingo = all([boards[board_idx][cell_idx][row_idx].mark
                                for cell_idx in range(0, board_size)])

                if any([col_bingo, row_bingo]):
                    winner_idxs.add(board_idx)
                    winners.append((num, boards[board_idx].copy()))
                    break

    return winners

with open("input.txt", "r") as file:
    contents = file.read().split('\n\n')

    nums: List[int] = [int(n) for n in contents[0].split(',')]
    boards: List[Board] = list(map(to_board, contents[1:]))

    bingos = find_bingos(boards, nums)
    assert len(bingos) > 0

    num_fst, board_fst = bingos[0]
    score_first = num_fst * sum([cell.num for row in board_fst for cell in row if not cell.mark])

    num_lst, board_lst = bingos[-1]
    score_last = num_lst * sum([cell.num for row in board_lst for cell in row if not cell.mark])

    print(f'{score_first=}')
    print(f'{score_last=}')
