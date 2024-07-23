import click
import pyperclip


@click.command(name="len")
@click.argument("input")
def length(input: str):
    res = len(input)

    click.echo(f"len: {click.style(res, fg="green", bold=True)},res has copy to your clipboard")
    pyperclip.copy(res)