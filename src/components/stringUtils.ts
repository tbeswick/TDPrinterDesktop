

export function format(template: string, ...args: unknown[]): string {
  return template.replace(/{(\d+)}/g, (_, index) => String(args[index]));
}

