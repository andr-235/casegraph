export function toggleSelectedId(current: string[], id: string): string[] {
  if (current.includes(id)) {
    return current.filter((item) => item !== id);
  }

  return [...current, id];
}
