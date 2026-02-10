function close_notification() {
    const callout = document.currentScript.closest('wa-callout');
    const close_button = callout.querySelector('wa-button.close');

    close_button.addEventListener('click', () => {
        const parent = callout.parentElement;

        if (parent?.localName === 'wa-animation') {
            parent.addEventListener('wa-finish', () => (parent.remove()));
            parent.play = true;
        } else {
            callout.remove();
        }
    });
}
