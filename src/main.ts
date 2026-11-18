import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import "./style.css";

const btn = document.getElementById("btn") as HTMLButtonElement;
const output = document.getElementById("output") as HTMLTextAreaElement;

btn.addEventListener("click", async () => {
  const file = await open({
    multiple: false,
    filters: [
      {
        name: "Imágenes",
        extensions: ["jpg", "jpeg", "png", "tiff", "bmp"],
      },
    ],
  });

  if (!file || Array.isArray(file)) return;

  const result = await invoke<[string, string][]>("leer_exif", {
    path: file,
  });

  output.value = result.map(([tag, value]) => `${tag}: ${value}`).join("\n");
  console.log("click");
  console.log("file:", file);
});
