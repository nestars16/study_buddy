
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

    
   const blobResponse = await serverResponse.blob();       

    const url = URL.createObjectURL(blobResponse);
    const anchor_download = document.createElement('a');
    anchor_download.href = url;
    anchor_download.download = 'StudyBuddyDownload.pdf' || 'download';

    anchor_download.click();
}
