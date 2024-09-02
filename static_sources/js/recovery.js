import {
  disableButtonAndShowSpinner,
  enableButtonAndRemoveSpinner,
  enableUserModalShake,
} from "./editorActions.js";

async function sendEmailForRecovery() {
  const emailModal = document.getElementById("email-submission");
  emailModal.classList.add("hidden");
  const sentModal = document.getElementById("sent");
  sentModal.classList.remove("hidden");
  await submitEmailButtonAction();
}

async function submitEmailButtonAction() {
  try {
    const serverResponse = await fetch("/send_recovery", {
      method: "POST",
      headers: {
        "Content-type": "application/json",
      },
      body: JSON.stringify({
        email: document.getElementById("text-field").value,
      }),
    });

    if (serverResponse.status != 200) {
      console.log("Failed to send email");
    }
  } catch (error) {
    console.log(error);
  }
}

async function tryRecoveryCode(recoveryCode, password, password_confirm) {
  const modalError = document.getElementById("modal-error");

  if (password !== password_confirm) {
    modalError.textContent = "Passwords must exactly match";
    enableButtonAndRemoveSpinner(
      document.getElementById("recovery-submit-button"),
    );
    enableUserModalShake(document.getElementById("recovery-modal"));
    return;
  }

  const passwordRegex = /(?=.*[A-Za-z])(?=.*\d).{8,}$/;

  if (!password.match(passwordRegex)) {
    modalError.textContent =
      "Password must contain minimum eight characters\nat least one letter and one number";
    enableButtonAndRemoveSpinner(
      document.getElementById("recovery-submit-button"),
    );
    enableUserModalShake(document.getElementById("recovery-modal"));
    return;
  }

  try {
    const serverResponse = await fetch("/try_recovery_code", {
      method: "POST",
      headers: {
        "Content-type": "application/json",
      },
      body: JSON.stringify({
        code: recoveryCode,
        password: password,
      }),
    });

    if (serverResponse.status != 200) {
      modalError.textContent = await serverResponse.text();
      enableButtonAndRemoveSpinner(
        document.getElementById("recovery-submit-button"),
      );
      enableUserModalShake(document.getElementById("recovery-modal"));
    }
  } catch (error) {
    modalError.textContent =
      "There was an error trying to reset your password try again in a few moments";
    enableButtonAndRemoveSpinner(
      document.getElementById("recovery-submit-button"),
    );
    enableUserModalShake(document.getElementById("recovery-modal"));
  }
}

function showRecoveryCodePrompt() {
  const emailModal = document.getElementById("email-submission");
  emailModal.close();

  const sentModal = document.getElementById("sent");
  sentModal.close();

  const recoveryModal = document.getElementById("recovery-modal");
  recoveryModal.show();
}

document.addEventListener("DOMContentLoaded", () => {
  const sendEmailForRecoveryButton = document.getElementById("submit-button");

  const recoveryCodeLink = document.getElementById("recovery-code");

  const codeRecoverySubmitButton = document.getElementById(
    "recovery-submit-button",
  );

  sendEmailForRecoveryButton.onclick = async () => {
    await sendEmailForRecovery();
  };
  recoveryCodeLink.onclick = () => {
    debugger;
    showRecoveryCodePrompt();
    debugger;
  };

  codeRecoverySubmitButton.onclick = async () => {
    disableButtonAndShowSpinner(codeRecoverySubmitButton);
    document.getElementById("modal-error").textContent = "";
    const code = document.getElementById("code-field").value;
    const password = document.getElementById("password").value;
    const password_confirm = document.getElementById(
      "password-confirmation",
    ).value;

    if (!code || !password || !password_confirm) {
      document.getElementById("modal-error").textContent =
        "All fields are required to be filled";
      enableButtonAndRemoveSpinner(codeRecoverySubmitButton);
      enableUserModalShake(document.getElementById("recovery-modal"));
      return;
    }

    await tryRecoveryCode(code, password, password_confirm);
  };
});
