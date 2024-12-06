
console.log('Hello world from main.js');

async function init() {
    const uploadInput = document.getElementById('upload');
    const newImgInput = document.getElementById('new-img');
    const fileReader = new FileReader();
    
    let rustApp = null;
    try {
        rustApp = await import('../pkg');
    } catch (error) {
        console.log(error);
        return;
    }

    console.log('rustApp: ', rustApp);

    uploadInput.addEventListener('change', () => {
        console.log('Image uploaded :>>', uploadInput.files[0]);
        fileReader.readAsDataURL(uploadInput.files[0]);
        console.time('Image processing');
    });

    fileReader.addEventListener('loadend', () => {
        const fileBase64 = fileReader.result
            .replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
        console.log('fileReader.addEventListener >> fileBase64 :>>', fileBase64);
        
        const imageDataUrl = rustApp.grayscale(fileBase64);
        console.timeEnd('Image processing');
        
        newImgInput.src = imageDataUrl;
    });
}

init();
