// method for encoding an Uint8Array to base64
function encode(input) {
    var keyStr = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    var output = "";
    var chr1, chr2, chr3, enc1, enc2, enc3, enc4;
    var i = 0;

    while (i < input.length) {
        chr1 = input[i++];
        chr2 = i < input.length ? input[i++] : Number.NaN; // Not sure if the index 
        chr3 = i < input.length ? input[i++] : Number.NaN; // checks are needed here

        enc1 = chr1 >> 2;
        enc2 = ((chr1 & 3) << 4) | (chr2 >> 4);
        enc3 = ((chr2 & 15) << 2) | (chr3 >> 6);
        enc4 = chr3 & 63;

        if (isNaN(chr2)) {
            enc3 = enc4 = 64;
        } else if (isNaN(chr3)) {
            enc4 = 64;
        }
        output += keyStr.charAt(enc1) + keyStr.charAt(enc2) +
            keyStr.charAt(enc3) + keyStr.charAt(enc4);
    }
    return output;
}

function resize(image, width, height) {
    return image.arrayBuffer().then((arrayBuffer) => {
        let imageUint8ClampedArray = new Uint8ClampedArray(arrayBuffer);
        return new Uint8ClampedArray(lib.resize(imageUint8ClampedArray, width, height));
    })
}

function compress(imageUint8ClampedArray) {
    return new Uint8ClampedArray(lib.compress(imageUint8ClampedArray));
}

function prepareDownloadButton(imageUint8Array, image_name, image_width, image_height) {
    console.log("finished");

    var base64encoded_image = encode(imageUint8Array);

    const linkSource = `data:image/png;base64,${base64encoded_image}`;

    var download_button = document.getElementById('download_button');
    download_button.style.display = 'block';
    download_button.classList.remove('invisible');
    download_button.classList.add('visible');

    download_button.addEventListener("click", function () {
        const downloadLink = document.createElement("a");
        downloadLink.href = linkSource;
        downloadLink.download = image_name.replace(/\.[^/.]+$/, "") + "_" + image_width + 'x' + image_height + ".png";
        downloadLink.click();
    });
}

(async () => {
    lib = await import("../pkg/index.js").catch(console.error);

    let image;

    window.addEventListener('load', function () {
        document.getElementById('run_button').onclick = function() {
            var width = parseInt(document.getElementById("resize_width").value);
            var height = parseInt(document.getElementById("resize_height").value);
            var shouldCompress = document.getElementById("yes_radio").checked;

            if(width && height) {
                // TODO: show spinning bar
                console.log("loading");

                // send image to resize function
                resize(image, width, height).then((resizedImageUint8Array) => {
                    if(shouldCompress) {
                        // send image to compress function
                        prepareDownloadButton(compress(resizedImageUint8Array), image.name, width, height);
                    } else {
                        prepareDownloadButton(resizedImageUint8Array, image.name, width, height);
                    }
                });
            }
        };

        document.getElementById('image-upload-button').addEventListener('change', function () {
            if (this.files && this.files[0]) {
                image = this.files[0];
                if (image.type !== "image/png") {
                    window.alert("Only PNG format is supported.")
                    image = null;
                    return;
                }
                document.getElementById("image_details").innerHTML = "Image Type: " + image.type.split("/").pop().toUpperCase();
            }
        });
    });
})()