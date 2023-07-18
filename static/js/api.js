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


