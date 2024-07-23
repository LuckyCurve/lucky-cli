import pathlib
from typing import Optional

import click


@click.command(name="size")
@click.option("-l", "--limit", required=False, type=int, help="limit show item", )
@click.option("--asc", required=False, type=bool, default=False, help="order by file size asc, default false")
def size(limit: Optional[int], asc: bool):
    """show current path file size order by file size desc"""
    _size(limit, asc)


def _size(limit: Optional[int], asc: bool):
    file_size_dice = {}
    current_path = pathlib.Path.cwd()

    _iter_path_file(current_path, file_size_dice, current_path)

    # handle sort
    res: list = sorted(file_size_dice.items(), key=lambda x: x[1], reverse=not asc)

    # limit
    if limit:
        res = res[:limit]

    [click.echo(f"{path}, {sizeof_fmt(size)}") for (path, size) in res]


def _iter_path_file(file: pathlib.Path, size_dict: dict, base_path: pathlib.Path):
    if file.is_file():
        file_size: int = file.stat().st_size
        file_related_path: str = str(file.relative_to(base_path))

        size_dict[file_related_path] = file_size
    else:
        for sub_file in file.iterdir():
            _iter_path_file(sub_file, size_dict, base_path)


def sizeof_fmt(num: int, suffix="B") -> str:
    for unit in ("", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"):
        if abs(num) < 1024.0:
            return f"{num:3.1f}{unit}{suffix}"
        num /= 1024.0
    return f"{num:.1f}Yi{suffix}"


if __name__ == '__main__':
    pass
