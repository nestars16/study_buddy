"use strict";


let url = new URL('/refresh', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');
var webSocketConnection = new WebSocket(url.href);

document.addEventListener("DOMContentLoaded", () => {
    


    let setup_mathjax = () => {
        window.MathJax = {
          tex: {
            inlineMath: [['$', '$'], ['\\(', '\\)']]
          },
          svg: {
            fontCache: 'global'
          }
        };
        
        let append_mathjax_script = () => {
          var script = document.createElement('script');
          script.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js';
          script.async = true;
          document.head.appendChild(script);
        };

        append_mathjax_script();
    }
        
    let setup_initial_md = async () => {

        let response  = await fetch("/api/md");

        if(response.status != 200) {
            throw new Error(`HTTP Error! status ${response.status}`)
        }

        let mdFilesJson = await response.json();

        for(const file of mdFilesJson) {

            let fileSidebar = document.getElementById("file-sidebar");           
            let liElement = document.createElement("a");
            liElement.textContent = file.name;

            fileSidebar.append(liElement);

            document.getElementById("markdown").innerHTML = file.file_content;

            MathJax.typeset();
        }

    }

    setup_mathjax();
    setup_initial_md();
})


document.addEventListener("keydown", () => {

    let markdown_content = document.getElementById("markdown").innerHTML;

    console.log(`Sending ${markdown_content}`);

    webSocketConnection.send(markdown_content);
})


webSocketConnection.addEventListener("message", (event) => {
    console.log(`recieving ${event.data}`);
})
