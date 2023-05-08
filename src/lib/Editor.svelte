<script lang="ts">
    import { readTextFile, writeFile} from "@tauri-apps/api/fs";
    import { save, open } from "@tauri-apps/api/dialog";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { EOL } from "@tauri-apps/api/os";

    

    async function handleOpenFile() {
    const result = await open({
     multiple:false,
     directory:false,
      filters: [{ name: "Text files", extensions: ["txt"] }, ]
    });

    if (result) {
      const fileContent = readTextFile(result.toString());
      const formattedContent = (await fileContent).replace(/(\r\n|\n|\r)/g, "<br>");
      const textEditor = document.querySelector(".texteditor");
      //Potential Security Risk, see about some means of sanitization that enables br tags to be preserved?
      textEditor.innerHTML = formattedContent.toString();
    }
  }

  async function saveFile()
  {
    //This regex and the use of innerhtml is necessary to get the linebreaks you typed in when you save a file. 
    var fileContent = document.querySelector(".texteditor").innerHTML.replace(/<br>|<div>/g, EOL).replace(/<[^>]+>/g, '')
    try
    {
      const result = await save({filters: [{name: "Text files", extensions: ['txt']}]})
      if (result)
      {
       writeFile(result, fileContent);
      }
    }
    catch (error) 
    {
      console.error('Error saving file:', error);
    }   
  }
    onMount(() => {
    listen("new-event", (event) => {
     document.querySelector(".texteditor").textContent = "";
    });

    listen("open-event", (event) => {
     handleOpenFile();
    });
    listen("save-event", (event) => {
      saveFile();
    })

  });
   
  </script>

  <div class = "texteditor" contenteditable="true">




  </div>

  <style>
    .texteditor
    {
        outline:none!important;
        height:100%;
    }

  </style>