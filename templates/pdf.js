const renderCallback = () => {
    renderMathInElement(document.querySelector("div"));
}

const script = document.createElement("script");
script.type = "text/javascript";
script.src = "https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/contrib/auto-render.min.js"

script.onload = renderCallback;
script.onreadystatechange = renderCallback;

document.head.appendChild(script);
