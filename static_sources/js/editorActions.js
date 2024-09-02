export async function savePost(documentId, text) {
  try {
    const response = await fetch("/save", {
      method: "PUT",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ document_id: documentId, text: text }),
    });

    if (response.status != 200) {
      open_external_error_modal(response, await response.text());
      return;
    }
  } catch (error) {
    open_external_error_modal(null, error);
  }
}

export async function deleteDocument(documentId) {
  try {
    const response = await fetch(`/delete_document?document_id=${documentId}`, {
      method: "DELETE",
      credentials: "include",
    });

    if (response.status != 200) {
      open_external_error_modal(response, await response.text());
      return;
    }
  } catch (error) {
    open_external_error_modal(null, error);
  }
}

export function highlight(editor, highlightEl) {
  window.requestAnimationFrame(() => {
    const highlighted = hljs.highlight(editor.value, {
      language: "markdown",
    }).value;
    highlightEl.innerHTML = highlighted;
  });
}

export function resizeTextarea(textArea) {
  if (!textArea) {
    return;
  }

  window.requestAnimationFrame(() => {
    textArea.style.height = 0;
    if (textArea.scrollHeight > 0) {
      textArea.style.height = `${textArea.scrollHeight + 2}px`;
    }
  });
}

export function enableTabbing(event) {
  if (event.key === "Tab") {
    event.preventDefault();

    editor.setRangeText(
      "  ",
      editor.selectionStart,
      editor.selectionStart,
      "end",
    );
  }
}

export function updateLineNumbers(event) {
  const lineNumbers = document.querySelector(".line-numbers");

  const numberOfLines = event.target.value.split("\n").length;

  lineNumbers.innerHTML = Array(numberOfLines).fill("<span></span>").join("");
}

export function closeDocumentCreationModal() {
  const titleModal = document.getElementById("user-document-title-modal");
  titleModal.close();
}

export function openDocumentTitleModal() {
  const titleModal = document.getElementById("user-document-title-modal");
  titleModal.showModal();
}

export function openUserActionsModal(modalTitle) {
  const modal = document.getElementById("user-modal");

  modal.showModal();

  const modal_h2 = document.getElementById("user-modal-title");
  modal_h2.textContent = modalTitle;

  const rememberMeText = document.getElementById("remember-me");
  const forgotPasswordLink = document.getElementById("forgot-password");

  switch (modalTitle) {
    case "Register":
      const confirmPassword = document.getElementById(
        "password-confirmation-field",
      );

      confirmPassword.classList.remove("hidden");
      rememberMeText.classList.add("hidden");
      forgotPasswordLink.classList.add("hidden");
      break;
    case "Log In":
      const toggleSwitch = document.getElementById("toggle-switch");
      toggleSwitch.classList.remove("hidden");
      rememberMeText.classList.remove("hidden");

      forgotPasswordLink.classList.remove("hidden");

      break;
  }

  modal.classList.remove("hidden");
}

export function closeUserActionModal() {
  const userActionModal = document.getElementById("user-modal");
  const confirmPassword = document.getElementById(
    "password-confirmation-field",
  );
  confirmPassword.classList.add("hidden");
  userActionModal.close();

  const toggleSwitch = document.getElementById("toggle-switch");

  toggleSwitch.classList.add("hidden");

  const rememberMeText = document.getElementById("remember-me");

  rememberMeText.classList.add("hidden");

  const forgotPasswordLink = document.getElementById("forgot-password");

  forgotPasswordLink.classList.add("hidden");
}

export function closeErrorModal() {
  const errorModal = document.getElementById("error-modal");
  errorModal.close();
}

export function closeAllDocumentsModal() {
  const allDocumentsModal = document.getElementById("all-documents-modal");
  allDocumentsModal.close();
}

export function toggleMode() {
  const body = document.querySelector("body");
  let currentMode = "";

  const toggleModeInner = (dark_variant, light_variant, element) => {
    if (element.classList.contains(dark_variant)) {
      element.classList.remove(dark_variant);
      element.classList.add(light_variant);
    } else {
      element.classList.remove(light_variant);
      element.classList.add(dark_variant);
    }
  };

  if (body.classList.contains("dark-mode-body")) {
    body.classList.remove("dark-mode-body");
    body.classList.add("light-mode-body");
    currentMode = "light";
  } else {
    body.classList.remove("light-mode-body");
    body.classList.add("dark-mode-body");
    currentMode = "dark";
  }

  const titles = document.querySelectorAll(".user-modal-title");

  for (const title of titles) {
    toggleModeInner("dark-user-modal-title", "light-user-modal-title", title);
  }

  const modals = document.querySelectorAll(".modal");

  for (const modal of modals) {
    toggleModeInner("dark-mode-modal", "light-mode-modal", modal);
  }

  const buttons = document.querySelectorAll(".action-button");

  for (const button of buttons) {
    if (button.id === "all-documents") {
      continue;
    }
    toggleModeInner("dark-mode-button", "light-mode-button", button);
  }

  const inputFields = [
    document.getElementById("email-field"),
    document.getElementById("password-field"),
    document.getElementById("password-confirmation-field"),
    document.getElementById("document-title-field"),
  ];

  for (const inputField of inputFields) {
    toggleModeInner(
      "dark-mode-text-field",
      "light-mode-text-field",
      inputField,
    );
  }

  const editor = document.getElementById("editor");
  toggleModeInner("dark-mode-input", "light-mode-input", editor);

  const toggleButton = document.getElementById("toggle-modes");
  const moonIcon = document.getElementById("moon");
  const sunIcon = document.getElementById("sun");

  if (toggleButton.classList.contains("dark-mode-toggle")) {
    toggleButton.classList.remove("dark-mode-toggle");
    toggleButton.classList.add("light-mode-toggle");
    moonIcon.classList.add("hidden");
    sunIcon.classList.remove("hidden");
  } else {
    toggleButton.classList.remove("light-mode-toggle");
    toggleButton.classList.add("dark-mode-toggle");
    sunIcon.classList.add("hidden");
    moonIcon.classList.remove("hidden");
  }

  return currentMode;
}

let globalTimeoutId = null;

async function changeToSelectedDocument(event, fetchFunction, documentArray) {
  if (globalTimeoutId) {
    clearInterval(globalTimeoutId);
  }

  const arrayId = event.target.id;
  const { document_id, title } = documentArray[arrayId];
  document.getElementById("document-title").innerText = title;
  const editor = document.getElementById("editor");
  const response = await fetchFunction(document_id);
  editor.value = response;
  document
    .getElementById("editor")
    .dispatchEvent(new Event("input", { bubbles: true }));
  document.getElementById("document-close-button").click();

  globalTimeoutId = setInterval(() => {
    savePost(document_id, document.getElementById("editor").value);
  }, 60000);
}

async function deleteSelectedDocument(event, deleteFunction, documentArray) {
  const arrayId = event.target.parentElement.id;

  const { document_id } = documentArray[arrayId];

  await deleteFunction(document_id);
}

export function showUserPosts(documentArray, currentMode, fetchFunction) {
  const documentModal = document.getElementById("document-section");
  documentModal.innerHTML = "";

  for (const [index, value] of documentArray.entries()) {
    const documentAnchor = document.createElement("a");
    const documentAnchorDelete = document.createElement("button");
    documentAnchorDelete.textContent = "🗑️";
    documentAnchorDelete.classList.add("button-delete");
    documentAnchor.href = "#";
    documentAnchor.id = index;

    let classListName;

    switch (currentMode) {
      case "dark":
        classListName = "dark-mode-document-link";
        break;
      case "light":
        classListName = "light-mode-document-link";
        break;
    }

    documentAnchor.classList.add(classListName);

    documentAnchor.onclick = (event) => {
      changeToSelectedDocument(event, fetchFunction, documentArray);
    };

    documentAnchorDelete.onclick = (event) => {
      deleteSelectedDocument(event, deleteDocument, documentArray);
      closeAllDocumentsModal();
    };

    documentAnchor.innerText = value.title;
    documentAnchor.appendChild(documentAnchorDelete);

    documentModal.appendChild(documentAnchor);

    if (index !== documentArray.length - 1) {
      documentModal.appendChild(document.createElement("hr"));
    }
  }

  document.getElementById("all-documents-modal").showModal();

  if (documentArray.length === 0) {
    documentModal.innerText =
      "You have no documents, try creating some with the plus icon 🤓";
  }
}

export function disableButtonAndShowSpinner(button, mode) {
  let spinnerColor;
  switch (mode) {
    case "light":
      spinnerColor = "black";
      break;
    case "dark":
      spinnerColor = "#FAFAFA";
      break;
  }

  button.style.borderTopColor = spinnerColor;
  button.disabled = true;
  button.classList.add("loading-button");
}

export function enableButtonAndRemoveSpinner(button) {
  button.disabled = false;
  button.classList.remove("loading-button");
}

export function enableLoadingScreen() {
  const overlay = document.querySelector(".overlay");
  overlay.classList.remove("hidden");
  overlay.classList.add("loading-overlay");
}

export function disableLoadingScreen() {
  const overlay = document.querySelector(".overlay");
  overlay.classList.add("hidden");
  overlay.classList.remove("loading-overlay");
}

export function enableUserModalShake(modal) {
  modal.classList.add("error-shake-modal");
}
