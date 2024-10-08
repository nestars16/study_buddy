import {
  enableButtonAndRemoveSpinner,
  closeDocumentCreationModal,
  enableUserModalShake,
} from "./editorActions.js";

export async function downloadMarkdownToPDF(html_body, css_stylings) {
  try {
    const serverResponse = await fetch("/download", {
      method: "POST",
      headers: {
        "Content-type": "application/json",
      },
      body: JSON.stringify({
        html: html_body,
        css: css_stylings,
      }),
    });

    const SUCCESS = 200;

    if (serverResponse.status != SUCCESS) {
      open_external_error_modal(serverResponse, await serverResponse.text());
      return;
    }

    const jsonResponse = await serverResponse.json();
    const anchor_download = document.createElement("a");
    anchor_download.href = jsonResponse.data.url;
    anchor_download.download = jsonResponse.data.url;
    anchor_download.target = "_blank";
    anchor_download.click();
    const downloadButton = document.getElementById("download");
    enableButtonAndRemoveSpinner(downloadButton);
  } catch (error) {
    open_external_error_modal(null, error);
  }
}

export function getCookiesObject() {
  const cookiesObject = document.cookie.split("; ").reduce((prev, current) => {
    const [name, ...value] = current.split("=");
    prev[name] = value.join("=");
    return prev;
  }, {});

  return cookiesObject;
}

export function checkForLogInUser() {
  const cookiesObject = getCookiesObject();

  document.getElementById("log-out").classList.add("hidden");
  document.getElementById("add-document").classList.add("hidden");
  document.getElementById("all-documents").classList.add("hidden");

  if (cookiesObject.session_id) {
    document.getElementById("sign-up").classList.add("hidden");
    document.getElementById("log-in").classList.add("hidden");
    document.getElementById("log-out").classList.remove("hidden");
    document.getElementById("add-document").classList.remove("hidden");
    document.getElementById("all-documents").classList.remove("hidden");
  }
}

export async function sendLogIn(username, password, wantsToBeRemembered) {
  const modalError = document.getElementById("modal-error");

  try {
    const response = await fetch("/log_in", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: username,
        password: password,
        wants_to_be_remembered: wantsToBeRemembered,
      }),
    });

    if (response.status != 200) {
      const responseText = await response.text();
      enableUserModalShake(document.getElementById("user-modal"));
      enableButtonAndRemoveSpinner(document.getElementById("submit-button"));
      modalError.textContent = responseText;
      return;
    }
  } catch (error) {
    open_external_error_modal(null, error);
  }

  location.reload();
}

export async function createUser(username, password, confirmPassword) {
  const modalError = document.getElementById("modal-error");

  const passwordRegex = /(?=.*[A-Za-z])(?=.*\d).{8,}$/;

  if (!password.match(passwordRegex)) {
    modalError.textContent =
      "Password must contain minimum eight characters\nat least one letter and one number";
    enableButtonAndRemoveSpinner(document.getElementById("submit-button"));
    enableUserModalShake(document.getElementById("user-modal"));
    return;
  }

  if (password !== confirmPassword) {
    modalError.textContent = "Passwords dont match";
    enableButtonAndRemoveSpinner(document.getElementById("submit-button"));
    enableUserModalShake(document.getElementById("user-modal"));
    return;
  }

  try {
    const response = await fetch("/create_user", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email: username, password: password }),
    });

    if (response.status != 201) {
      const responseText = await response.text();
      enableButtonAndRemoveSpinner(document.getElementById("submit-button"));
      enableUserModalShake(document.getElementById("user-modal"));
      modalError.textContent = responseText;
      return;
    }

    location.reload();
  } catch (error) {
    open_external_error_modal(null, error);
  }
}

export async function submitButtonAction() {
  document.getElementById("modal-error").textContent = "";

  const modalType = document.querySelector(".user-modal-title").textContent;
  const email = document.getElementById("email-field").value;
  const password = document.getElementById("password-field").value;
  const errorMessage = document.getElementById("modal-error");
  const fieldsArentFilled = !(email && password);

  switch (modalType) {
    case "Log In":
      const wantsToBeRemembered =
        document.querySelector(".toggle__input").checked;

      if (fieldsArentFilled) {
        errorMessage.textContent = "All fields are required";
        enableButtonAndRemoveSpinner(document.getElementById("submit-button"));
        enableUserModalShake(document.getElementById("user-modal"));
        return;
      }
      await sendLogIn(email, password, wantsToBeRemembered);
      break;
    case "Register":
      const confirmPassword = document.getElementById(
        "password-confirmation-field",
      ).value;

      if (fieldsArentFilled || !confirmPassword) {
        errorMessage.textContent = "All fields are required";
        enableUserModalShake(document.getElementById("user-modal"));
        enableButtonAndRemoveSpinner(document.getElementById("submit-button"));
        return;
      }

      await createUser(email, password, confirmPassword);
      break;
  }

  const submitButton = document.getElementById("submit-button");
  enableButtonAndRemoveSpinner(submitButton);
}

export async function logOut() {
  try {
    const response = await fetch("/log_out", {
      method: "POST",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({}),
    });

    if (response.status != 200) {
      open_external_error_modal(response, await response.text());
      enableButtonAndRemoveSpinner(document.getElementById("log-out"));
      return;
    }

    location.reload();
  } catch (error) {
    enableButtonAndRemoveSpinner(document.getElementById("log-out"));
    open_external_error_modal(null, error);
  }
}

export async function createDocument(title) {
  try {
    const response = await fetch("/create_document", {
      method: "POST",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ title: title }),
    });

    if (response.status != 200) {
      document
        .getElementById("user-document-title-modal")
        .classList.add("hidden");
      document.querySelector(".overlay").classList.remove("hidden");
      open_external_error_modal(response, await response.text());
      return;
    }

    const jsonResponse = await response.json();
    console.log(jsonResponse);

    document.getElementById("document-title").textContent = title;
  } catch (error) {
    open_external_error_modal(null, error);
  }

  closeDocumentCreationModal();
  const submit = document.getElementById("document-title-submit");
  enableButtonAndRemoveSpinner(submit);
}

export async function fetchUserDocuments() {
  try {
    const response = await fetch("/fetch_documents", {
      method: "GET",
      credentials: "include",
    });

    if (response.status != 200) {
      open_external_error_modal(response, await response.text());
      return;
    }

    return await response.json();
  } catch (error) {
    open_external_error_modal(null, error);
  }
}

export async function fetchCurrentDocumentContent(doc_id) {
  const URL = `/fetch_content?document_id=${doc_id}`;

  try {
    const response = await fetch(URL, {
      method: "GET",
      credentials: "include",
    });

    if (response.status != 200) {
      open_external_error_modal(response, await response.text());
      return;
    }

    return await response.json();
  } catch (error) {
    open_external_error_modal(null, error);
  }
}

function open_external_error_modal(serverResponse, text) {
  if (!serverResponse) {
    serverResponse.status = "No status code";
  }

  const errorModal = document.getElementById("error-modal");

  errorModal.show();

  document.getElementById("error-message").textContent =
    `Error code : ${serverResponse.status} - ${text}`;
}
