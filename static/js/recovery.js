const sendEmailForRecovery = async () => {
    const emailModal = document.getElementById("email-submission");
    emailModal.classList.add("hidden");
    const sentModal = document.getElementById("sent");
    sentModal.classList.remove("hidden");
    await submitEmailButtonAction();
}

const submitEmailButtonAction = async () => {

}

document.addEventListener("DOMContentLoaded", () => {
    const sendEmailForRecoveryButton = document.getElementById("submit-button");
    sendEmailForRecoveryButton.onclick = async () => {
        debugger;
        await sendEmailForRecovery();
    }
})
