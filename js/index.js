"use strict";

document.addEventListener("DOMContentLoaded", () => {
        
    let setup = async () => {

    window.MathJax = {
      tex: {
        inlineMath: [['$', '$'], ['\\(', '\\)']]
      },
      svg: {
        fontCache: 'global'
      }
    };
    
    (function () {
      var script = document.createElement('script');
      script.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js';
      script.async = true;
      document.head.appendChild(script);
    })();

        let response  = await fetch("/api/md");

        if(response.status != 200) {
            throw new Error(`HTTP Error! status ${response.status}`)
        }

        let mdFilesJson = await response.json();

        for(const file of mdFilesJson) {
            console.log(file);
            let fileSidebar = document.getElementById("file-sidebar");           
            let liElement = document.createElement("li");
            liElement.textContent = file.name;

            fileSidebar.append(liElement);

            document.getElementById("markdown").innerHTML = file.file_content;

            MathJax.typeset();
        }

    }

    setup();

})
