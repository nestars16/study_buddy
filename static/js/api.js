
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

    console.log(jsonResponse);

    const anchor_download = document.createElement('a');
    anchor_download.href = jsonResponse.data.url;
    anchor_download.download = 'StudyBuddyDownload.pdf' ;
    anchor_download.target = "_blank";
    anchor_download.click();

}


export const open_modal = (modal_title,display) => {  

    const modal = document.querySelector(".modal");
    const overlay = document.querySelector(".overlay");
    const modal_h2 = document.querySelector(".user-modal-title");
    const editor = document.querySelector(".editor-container"); 

    display.classList.add("hidden");
    editor.classList.add("hidden");

    modal_h2.textContent = modal_title;

    overlay.classList.remove("hidden");
    modal.classList.remove("hidden");
}

export const closeModal = (display) => {

    console.log("clicked close");
    const modal = document.querySelector(".modal");
    const overlay = document.querySelector(".overlay");
    const editor = document.querySelector(".editor-container"); 

    display.classList.remove("hidden");
    editor.classList.remove("hidden");

    overlay.classList.add("hidden");
    modal.classList.add("hidden");
}

export const sendLogIn = async (username, password) => {
    
    const response = await fetch("/log_in", {
        method: "POST",
        headers : {
            "Content-Type" : "application/json",
        },
        body : JSON.stringify({ email : username , password : password})
    });


    const jsonResponse = await response.json();

    console.log(jsonResponse);
}


export const createUser = async (username,password) => {


    const response = await fetch("/create_user", {
        method: "POST",
        headers : {
            "Content-Type" : "application/json",
        },
        body : JSON.stringify({ email : username , password : password})
    });

    const jsonResponse = await response.json();

    console.log(jsonResponse);
}
