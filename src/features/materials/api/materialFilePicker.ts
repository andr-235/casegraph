import { open } from "@tauri-apps/plugin-dialog";

export async function pickMaterialFile(): Promise<string | null> {
  const selected = await open({
    multiple: false,
    directory: false,
    title: "Выберите материал",
    filters: [
      {
        name: "Поддерживаемые материалы",
        extensions: [
          "png",
          "jpg",
          "jpeg",
          "pdf",
          "docx",
          "txt",
          "xlsx",
          "csv",
          "html",
          "htm",
        ],
      },
      {
        name: "Все файлы",
        extensions: ["*"],
      },
    ],
  });

  if (!selected) {
    return null;
  }

  if (Array.isArray(selected)) {
    return selected[0] ?? null;
  }

  return selected;
}
