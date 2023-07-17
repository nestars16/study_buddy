
 const open_external_error_modal = () => {

        const display = document.getElementById("markdown-display");  
        const errorModal = document.getElementById("error-modal");
        const overlay = document.querySelector(".overlay");
        const editor = document.querySelector(".editor-container"); 

        display.classList.add("hidden");
        editor.classList.add("hidden");

        overlay.classList.remove("hidden");
        errorModal.classList.remove("hidden");

        document.getElementById("error-message").textContent = 
            `Error code : ${serverResponse.status} - ${serverResponse.type}`;

 }


export const open_modal = (modalTitle,display) => {  

    const modal = document.querySelector(".modal");
    const overlay = document.querySelector(".overlay");
    const modal_h2 = document.querySelector(".user-modal-title");
    const editor = document.querySelector(".editor-container"); 

    document.getElementById("submit-button").classList.remove("hidden");
    document.getElementById("loader").classList.add("hidden");

    display.classList.add("hidden");
    editor.classList.add("hidden");

    modal_h2.textContent = modalTitle;

    if (modalTitle === "Register") {
        const confirmPassword = document.getElementById("password-confirmation-field");
        confirmPassword.classList.remove("hidden");
    }

    overlay.classList.remove("hidden");
    modal.classList.remove("hidden");
}

export const closeModal = (display) => {

    console.log("clicked close");
    const modal = document.querySelector(".modal");
    const errorModal = document.getElementById("error-modal");
    const overlay = document.querySelector(".overlay");
    const editor = document.querySelector(".editor-container"); 
    const confirmPassword = document.getElementById("password-confirmation-field");
    const errorMessage = document.getElementById("modal-error");

    display.classList.remove("hidden");
    editor.classList.remove("hidden");

    overlay.classList.add("hidden");
    modal.classList.add("hidden");
    errorModal.classList.add("hidden");
    confirmPassword.classList.add("hidden");
    errorMessage.textContent = '';
}

export const toggleMode = () => {
   
    const body = document.querySelector("body");
    let currentMode = '';

    if (body.classList.contains("dark-mode-body")) {
        body.classList.remove("dark-mode-body");
        body.classList.add("light-mode-body");
        currentMode = 'light';
    } else {
        body.classList.remove("light-mode-body");
        body.classList.add("dark-mode-body");
        currentMode = 'dark';
    }

    const modal = document.getElementById("main-modal");

    if (modal.classList.contains("dark-mode-modal")) {
        modal.classList.remove("dark-mode-modal");
        modal.classList.add("light-mode-modal");
    } else {
        modal.classList.remove("light-mode-modal");
        modal.classList.add("dark-mode-modal");
    }

    const buttons = document.querySelectorAll(".action-button");

    for(let button of buttons) {
        if (button.classList.contains("dark-mode-button")) {
            button.classList.remove("dark-mode-button");
            button.classList.add("light-mode-button");
        } else {
            button.classList.remove("light-mode-button");
            button.classList.add("dark-mode-button");
        }
    }
    
    const inputFields = [document.getElementById("email-field"), document.getElementById("password-field"), document.getElementById("password-confirmation-field")];

    for(let inputField of inputFields) {
        if (inputField.classList.contains("dark-mode-text-field")) {
            inputField.classList.remove("dark-mode-text-field");
            inputField.classList.add("light-mode-text-field");
        } else {
            inputField.classList.remove("light-mode-text-field");
            inputField.classList.add("dark-mode-text-field");
        }
    }

    const editor = document.getElementById("editor");

    if(editor.classList.contains("dark-mode-input")) {
        editor.classList.remove("dark-mode-input");
        editor.classList.add("light-mode-input");
    }else {
        editor.classList.remove("light-mode-input");
        editor.classList.add("dark-mode-input");
    }

    const toggleButton = document.getElementById("toggle-modes");

    const moonIcon = document.getElementById("moon");
    const sunIcon = document.getElementById("sun");

    if(toggleButton.classList.contains("dark-mode-toggle")) {
        toggleButton.classList.remove("dark-mode-toggle");
        toggleButton.classList.add("light-mode-toggle");
        moonIcon.classList.add("hidden");
        sunIcon.classList.remove("hidden");
    }else {
        toggleButton.classList.remove("light-mode-toggle");
        toggleButton.classList.add("dark-mode-toggle");
        sunIcon.classList.add("hidden");
        moonIcon.classList.remove("hidden");
    }

    return currentMode;
}

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

    const SUCCESS = 200;

    if (serverResponse.status != SUCCESS) { 

        open_external_error_modal();

        return;                
        
    }

    const jsonResponse = await serverResponse.json();

    console.log(jsonResponse);

    const anchor_download = document.createElement('a');
    anchor_download.href = jsonResponse.data.url;
    anchor_download.download = jsonResponse.data.url;
    anchor_download.target = "_blank";
    anchor_download.click();
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

    }

}

export const sendLogIn = async (username, password) => {

    const modalError = document.getElementById("modal-error");  
    const display = document.getElementById("markdown-display");  

    document.getElementById("submit-button").classList.add("hidden"); 
    document.getElementById("loader").classList.remove("hidden"); 

    const response = await fetch("/log_in", {
        method: "POST",
        headers : {
            "Content-Type" : "application/json",
        },
        body : JSON.stringify({ email : username , password : password})
    });


    if (response.status != 200) {

        const responseText = await response.text();

        document.getElementById("submit-button").classList.remove("hidden");
        document.getElementById("loader").classList.add("hidden"); 

        modalError.textContent = responseText;
        return;
    }


    console.log("Logged in");
    closeModal(display);

    document.getElementById("submit-button").classList.remove("hidden");
    document.getElementById("loader").classList.add("hidden"); 
    location.reload();
}


export const createUser = async (username,password,confirmPassword) => {

    const modalError = document.getElementById("modal-error");  
    const display = document.getElementById("markdown-display");  


    if(password !== confirmPassword) {
        modalError.textContent = "Passwords dont match";     

        return;
    }

    document.getElementById("submit-button").classList.add("hidden"); 
    document.getElementById("loader").classList.remove("hidden"); 

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

    console.log("Created User");
    closeModal(display);

    document.getElementById("submit-button").classList.remove("hidden");
    document.getElementById("loader").classList.add("hidden"); 

    location.reload();
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
}

export const LogOut = async () => {

    const cookiesObject = getCookiesObject();

    const response = await fetch("/log_out", 
        {
            method : "POST",
            headers : {
                "Content-Type" : "application/json"
            },
            body : JSON.stringify({session_id : cookiesObject.session_id })
        })

    if (response.status  != 200) {
        open_external_error_modal(); 
        return;
    }

    location.reload();
}


