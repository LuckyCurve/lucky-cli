import click

import file.info
import myjson.commands
import text.commands


@click.group
def root():
    """
    lucky command util for enhance your work and life
    """
    pass


@root.group(name="json")
def json_group():
    """
    json group for operate json data
    """
    pass


@root.group(name="file")
def file_group():
    pass


# noinspection PyTypeChecker
root.add_command(text.commands.length)
json_group.add_command(myjson.commands.format_command)
file_group.add_command(file.info.size)

if __name__ == '__main__':
    root()
