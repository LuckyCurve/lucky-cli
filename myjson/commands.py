import click


@click.command(name="format")
@click.argument("input")
def format_command(input: str):
    click.echo(f"{input}")