const pyrin = require('../../../../nodejs/pyrin');

pyrin.initConsolePanicHook();

(async () => {

    let encrypted = pyrin.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = pyrin.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
