
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

    const documentBar = document.getElementById("document-bar");
    hideMainContentAndShowOverlay();

    const titleModal = document.getElementById("user-document-title-modal");
    titleModal.classList.remove("hidden");

}

export const openModal = (modalTitle) => {  

    hideMainContentAndShowOverlay();

    const modal = document.querySelector(".modal");
    const modal_h2 = document.querySelector(".user-modal-title");

    document.getElementById("submit-button").classList.remove("hidden");
    document.getElementById("loader").classList.add("hidden");

    modal_h2.textContent = modalTitle;

    if (modalTitle === "Register") {
        const confirmPassword = document.getElementById("password-confirmation-field");
        confirmPassword.classList.remove("hidden");
    }

    modal.classList.remove("hidden");
}

export const closeModal = () => {

    console.log("clicked close");

    const display = document.getElementById("markdown-display");
    const modal = document.querySelector(".modal");
    const errorModal = document.getElementById("error-modal");
    const overlay = document.querySelector(".overlay");
    const editor = document.querySelector(".editor-container"); 
    const confirmPassword = document.getElementById("password-confirmation-field");
    const errorMessage = document.getElementById("modal-error");
    const titleModal = document.getElementById("user-document-title-modal");

    titleModal.classList.add("hidden");
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

    const titles = document.querySelectorAll(".user-modal-title");

    for(let title of titles) {
        if(title.classList.contains("dark-user-modal-title")){
            title.classList.remove("dark-user-modal-title");
            title.classList.add("light-user-modal-title");
        }else {
            title.classList.add("dark-user-modal-title");
            title.classList.remove("light-user-modal-title");
        }
    }

    const modals = document.querySelectorAll(".modal");

    for(let modal of modals) {
        if (modal.classList.contains("dark-mode-modal")) {
            modal.classList.remove("dark-mode-modal");
            modal.classList.add("light-mode-modal");
        } else {
            modal.classList.remove("light-mode-modal");
            modal.classList.add("dark-mode-modal");
        }
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
    
    const inputFields = [document.getElementById("email-field"), document.getElementById("password-field"), document.getElementById("password-confirmation-field"), 
        document.getElementById("document-title-field")];

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

