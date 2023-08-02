import { enableButtonAndRemoveSpinner, closeDocumentCreationModal} from "./editorActions.js";


export const downloadMarkdownToPDF = async (html_body,css_stylings) => {
    
    try {
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

        const SUCCESS = 200;

        if (serverResponse.status != SUCCESS) { 
            open_external_error_modal(serverResponse, await serverResponse.text());
            return;                
        }

        const jsonResponse = await serverResponse.json();
        const anchor_download = document.createElement('a');
        anchor_download.href = jsonResponse.data.url;
        anchor_download.download = jsonResponse.data.url;
        anchor_download.target = "_blank";
        anchor_download.click();
        const downloadButton = document.getElementById("download");
        enableButtonAndRemoveSpinner(downloadButton); 

    } catch(error) {
        open_external_error_modal(null, error);
    }
}

export const getCookiesObject = () => {

        const cookiesObject = document.cookie.split('; ').reduce(
            (prev, current) => {
                const [name, ...value] = current.split('=');
                prev[name] = value.join('=');
                return prev;
            }, {}); 

    return cookiesObject;
}

export const checkForLogInUser = () => {

    const cookiesObject = getCookiesObject(); 

    if (cookiesObject.session_id) {
        document.getElementById("sign-up").classList.add("hidden")
        document.getElementById("log-in").classList.add("hidden")
        document.getElementById("log-out").classList.remove("hidden");
        document.getElementById("add-document").classList.remove("hidden");
        document.getElementById("all-documents").classList.remove("hidden");
    }
}

export const sendLogIn = async (username, password) => {

    const modalError = document.getElementById("modal-error");  

    try {
        const response = await fetch("/log_in", {
            method: "POST",
            headers : {
                "Content-Type" : "application/json",
            },
            body : JSON.stringify({ email : username , password : password})
        });


        if (response.status != 200) {
            const responseText = await response.text();


            modalError.textContent = responseText;
            return;
        }
    }catch(error) {
        open_external_error_modal(null, error);
    }

    location.reload();
}


export const createUser = async (username,password,confirmPassword) => {

    const modalError = document.getElementById("modal-error");  

    if(password !== confirmPassword) {
        modalError.textContent = "Passwords dont match";     

        return;
    }

    document.getElementById("submit-button").classList.add("hidden"); 
    document.getElementById("loader").classList.remove("hidden"); 

    try {
        const response = await fetch("/create_user", {
            method: "POST",
            headers : {
                "Content-Type" : "application/json",
            },
            body : JSON.stringify({ email: username ,password : password})
        });


        if(response.status != 201) {
            const responseText = await response.text();

            document.getElementById("submit-button").classList.remove("hidden");
            document.getElementById("loader").classList.add("hidden"); 

            modalError.textContent = responseText;
            return;
        }

        location.reload();

    }catch(error) {
        open_external_error_modal(null,error);
    }
}

export const submitButtonAction = async () => {

            document.getElementById("modal-error").textContent = '';

            const modalType = document.querySelector(".user-modal-title").textContent;
            const email = document.getElementById("email-field").value;
            const password = document.getElementById("password-field").value;
            const confirmPassword =  document.getElementById("password-confirmation-field").value;

            console.log(modalType);

            switch(modalType){
                case "Log In":
                       await sendLogIn(email,password); 
                    break;
                case "Register":
                        await createUser(email,password,confirmPassword);
                    break;
            }

            const submitButton = document.getElementById("submit-button");
            enableButtonAndRemoveSpinner(submitButton);
}

export const logOut = async () => {

    try {
        const response = await fetch("/log_out", 
            {
                method : "POST",
                credentials : "include",
                headers : {
                    "Content-Type" : "application/json"
                },
                body : JSON.stringify({})
            })

        if (response.status  != 200) {
            open_external_error_modal(response, await response.text()); 
            return;
        }

        location.reload();
    } catch(error) {
        open_external_error_modal(null, error);
    }
}

export const createDocument = async (title) => {

    try {
        const response = await fetch("/create_document", {
            method : "POST",
            credentials: "include",
            headers : {
                "Content-Type" : "application/json"
            },
            body : JSON.stringify({title : title})
        });

        if (response.status != 200) {
            document.getElementById("user-document-title-modal").classList.add("hidden");
            document.querySelector(".overlay").classList.remove("hidden");
            open_external_error_modal(response, await response.text()); 
            return;
        }
        
        const jsonResponse = await response.json();
        console.log(jsonResponse);

        document.getElementById("document-title").textContent = title;

    }catch(error) {
        open_external_error_modal(null, error);
    }

    closeDocumentCreationModal();
    const submit = document.getElementById("document-title-submit");
    enableButtonAndRemoveSpinner(submit);
}

export const fetchUserDocuments  = async () => {

    try {
        const response = await fetch("/fetch_documents", {
            method : "GET",
            credentials : "include"
        });

        if (response.status != 200) {
            open_external_error_modal(response, await response.text());
            return;
        }

        return await response.json();
    }catch(error) {
        open_external_error_modal(null, error);
    }
}

export const fetchCurrentDocumentContent = async (doc_id) => {

    const URL = `/fetch_content?document_id=${doc_id}`;

    try {
        const response = await fetch(URL, {
            method : "GET",
            credentials : "include"

        });

        if (response.status != 200) {
            open_external_error_modal(response, await response.text());
            return;
        }

        return await response.json();
    } catch(error) {
        open_external_error_modal(null, error);
    }
}

export const savePost = async (document_id, text) => {
    
    try {
        const response = await fetch("/save",  {
            method : "PUT",
            credentials : "include",
            headers : {
                "Content-Type" : "application/json"
            },
            body : JSON.stringify({document_id : document_id, text: text})
        });

        if(response.status != 200) {
            open_external_error_modal(response, await response.text());
            return;
        }

    }catch(error) {
        open_external_error_modal(null, error);
    }

}

 function open_external_error_modal (serverResponse, text) {

        if(!serverResponse) {
            serverResponse.status = "No status code";
        }

        const display = document.getElementById("markdown-display");  
        const errorModal = document.getElementById("error-modal");
        const overlay = document.querySelector(".overlay");
        const editor = document.querySelector(".editor-container"); 

        display.classList.add("hidden");
        editor.classList.add("hidden");

        overlay.classList.remove("hidden");
        errorModal.classList.remove("hidden");

        document.getElementById("error-message").textContent = 
            `Error code : ${serverResponse.status} - ${text}`;

 }
