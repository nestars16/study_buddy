import { savePost } from "./api.js";


export const highlight = (editor, highlightEl) => {
  window.requestAnimationFrame(() => {
    const highlighted = hljs.highlight(
      editor.value, 
        {language : "markdown"}
    ).value;
    highlightEl.innerHTML = highlighted;
  });
};

export const resizeTextarea = (textArea) => {
  if (!textArea) {
    return;
  }


  window.requestAnimationFrame(() => {
    textArea.style.height = 0;
    if (textArea.scrollHeight > 0) {
      textArea.style.height = `${textArea.scrollHeight + 2}px`;
    }
  });
};

export const enableTabbing =  (event) => {
            if (event.key === 'Tab') {
                event.preventDefault()

                editor.setRangeText(
                  '  ',
                  editor.selectionStart,
                  editor.selectionStart,
                  'end'
                )
              }
        }

export const updateLineNumbers = (event) => {
    const lineNumbers = document.querySelector(".line-numbers");

    const numberOfLines = event.target.value.split('\n').length

    lineNumbers.innerHTML = Array(numberOfLines)
        .fill('<span></span>')
        .join('')
}

const hideMainContentAndShowOverlay = () => {

    const display = document.getElementById("markdown-display");
    const editor = document.querySelector(".editor-container"); 

    const overlay = document.querySelector(".overlay");

    overlay.classList.remove("hidden");
    display.classList.add("hidden");
    editor.classList.add("hidden");
}



export const openDocumentTitleModal = () => {
    hideMainContentAndShowOverlay();
    const titleModal = document.getElementById("user-document-title-modal");
    titleModal.classList.remove("hidden");
}

export const openUserActionsModal = (modalTitle) => {  

    hideMainContentAndShowOverlay();

    const modal = document.getElementById("user-modal");
    const modal_h2 = document.getElementById("user-modal-title");
    modal_h2.textContent = modalTitle;

    switch(modalTitle) {
        case "Register":
            const confirmPassword = document.getElementById("password-confirmation-field");
            confirmPassword.classList.remove("hidden");
            break;
        case "Log In":
            const toggleSwitch = document.getElementById("toggle-switch");
            toggleSwitch.classList.remove("hidden");
            const rememberMeText = document.getElementById("remember-me");
            rememberMeText.classList.remove("hidden");
            const forgotPasswordLink = document.getElementById("forgot-password");
            forgotPasswordLink.classList.remove("hidden");
            break;
    }

    debugger;

    modal.classList.remove("hidden");
}

export const hideOverlayAndShowMainContent = () => {

    const display = document.getElementById("markdown-display");
    const editor = document.querySelector(".editor-container"); 
    const overlay = document.querySelector(".overlay");
    const errorMessage = document.getElementById("modal-error");

    display.classList.remove("hidden");
    editor.classList.remove("hidden");
    overlay.classList.add("hidden");

    errorMessage.textContent = '';
}

export const closeUserActionModal = () => {
    hideOverlayAndShowMainContent();

    const userActionModal = document.getElementById("user-modal");
    const confirmPassword = document.getElementById("password-confirmation-field");
    confirmPassword.classList.add("hidden");
    userActionModal.classList.add("hidden");
    const toggleSwitch = document.getElementById("toggle-switch");
    toggleSwitch.classList.add("hidden");
    const rememberMeText = document.getElementById("remember-me");
    rememberMeText.classList.add("hidden");
    const forgotPasswordLink = document.getElementById("forgot-password");
    forgotPasswordLink.classList.add("hidden");
}

export const closeErrorModal = () => {
    hideOverlayAndShowMainContent();
    const errorModal = document.getElementById("error-modal");
    errorModal.classList.add("hidden");
}

export const closeDocumentCreationModal = () => {
    hideOverlayAndShowMainContent();
    const titleModal = document.getElementById("user-document-title-modal");
    titleModal.classList.add("hidden");
}

export const closeAllDocumentsModal = () => {
    hideOverlayAndShowMainContent();
    const allDocumentsModal = document.getElementById("all-documents-modal");
    allDocumentsModal.classList.add("hidden");
}

export const toggleMode = () => {
    const body = document.querySelector("body");
    let currentMode = '';

    const toggleModeInner = (dark_variant, light_variant, element) => {
        if(element.classList.contains(dark_variant)) {
            element.classList.remove(dark_variant);
            element.classList.add(light_variant);
        }else {
            element.classList.remove(light_variant);
            element.classList.add(dark_variant);
        }
    }

    if (body.classList.contains("dark-mode-body")) {
        body.classList.remove("dark-mode-body");
        body.classList.add("light-mode-body");
        currentMode = 'light';
    } else {
        body.classList.remove("light-mode-body");
        body.classList.add("dark-mode-body");
        currentMode = 'dark';
    }

    const titles = document.querySelectorAll(".user-modal-title");

    for(const title of titles) {
        toggleModeInner("dark-user-modal-title", "light-user-modal-title", title);
    }

    const modals = document.querySelectorAll(".modal");

    for(const modal of modals) {
        toggleModeInner("dark-mode-modal", "light-mode-modal", modal)
    }

    const buttons = document.querySelectorAll(".action-button");

    for(const button of buttons) {

        if(button.id === "all-documents") {
            continue;
        }
        toggleModeInner("dark-mode-button","light-mode-button", button);
    }
    
    const inputFields = [document.getElementById("email-field"), document.getElementById("password-field"), document.getElementById("password-confirmation-field"), 
        document.getElementById("document-title-field")];

    for(const inputField of inputFields) {
        toggleModeInner("dark-mode-text-field", "light-mode-text-field", inputField)
    }

    const editor = document.getElementById("editor");
    toggleModeInner("dark-mode-input", "light-mode-input", editor);

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

const changeToSelectedDocument = async (event, fetchFunction,documentArray, globalCurrentTimeoutId) => {

    if(globalCurrentTimeoutId.current_id) {
        debugger;
        clearInterval(globalCurrentTimeoutId);
    }

    const arrayId = event.target.id;
    const {document_id, title} = documentArray[arrayId];
    document.getElementById("document-title").innerText = title; 
    const editor = document.getElementById("editor");
    const response = await fetchFunction(document_id);
    editor.value = response; 
    document.getElementById("editor").dispatchEvent(new Event('input', { bubbles: true }));
    document.getElementById("document-close-button").click();

    globalCurrentTimeoutId.current_id = setInterval(() => {
        debugger;
        savePost(document_id, document.getElementById("editor").value)
    }, 60000)
}

export const showUserPosts = (documentArray, currentMode, fetchFunction, globalCurrentTimeoutId) => {

    const documentModal = document.getElementById("document-section");
    documentModal.innerHTML = '';

    for(const [index, value] of documentArray.entries()) {
        const documentAnchor = document.createElement("a");
        documentAnchor.href = "#";
        documentAnchor.id = index;

        let classListName;

        switch(currentMode) {
            case "dark" :
                classListName = "dark-mode-document-link";
                break;
            case "light":
                classListName = "light-mode-document-link";
                break;
        }

        documentAnchor.classList.add(classListName)

        documentAnchor.onclick = (event) => {
            changeToSelectedDocument(event,fetchFunction,documentArray,globalCurrentTimeoutId);
        }

        documentAnchor.innerText = value.title;
    
        documentModal.appendChild(documentAnchor);

        if(index !== documentArray.length - 1) {
            documentModal.appendChild(document.createElement("hr"));
        }

    }
    document.getElementById("all-documents-modal").classList.remove("hidden");
    hideMainContentAndShowOverlay();
}

export const disableButtonAndShowSpinner = (button,mode) => {

    let spinnerColor;
    switch(mode) {
        case "light":
            spinnerColor = "black"
            break;
        case "dark":
            spinnerColor = "#FAFAFA";
            break;
    }

    button.style.borderTopColor = spinnerColor;
    button.disabled = true;
    button.classList.add("loading-button");    
}

export const enableButtonAndRemoveSpinner = (button)=> {
    button.disabled = false;
    button.classList.remove("loading-button");    
}


export const enableLoadingScreen = () => {
    const  overlay = document.querySelector(".overlay");
    overlay.classList.remove("hidden");
    overlay.classList.add("loading-overlay");
}

export const disableLoadingScreen = () => {
    const  overlay = document.querySelector(".overlay");
    overlay.classList.add("hidden");
    overlay.classList.remove("loading-overlay");
}

export const enableUserModalShake = () => {

    const userModal = document.getElementById("user-modal");    
    userModal.classList.add("error-shake-modal");

}
