
export const downloadMarkdownToPDF = async (html_body,css_stylings) => {
    
    const serverResponse = await fetch("/download", {
        method : "POST",
        headers : {
            "Content-type": "application/json",
        },
        body : JSON.stringify({
            html : html_body,
            css : css_stylings,
        })
    });

    const jsonResponse = await serverResponse.json();

    const anchor_download = document.createElement('a');
    anchor_download.href = jsonResponse.data.url;
    anchor_download.download = 'StudyBuddyDownload.pdf' ;
    anchor_download.target = "_blank";
    anchor_download.click();

}
