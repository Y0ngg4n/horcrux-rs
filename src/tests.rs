#[cfg(test)]
mod tests {
    use std::fs::{self, File, OpenOptions};
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use chacha20poly1305::aead::OsRng;
    use rand::RngCore;
    // Import the necessary modules and functions you want to test
    use sha2::{Digest, Sha256};

    use crate::commands::{bind::bind, split::split};
    use crate::crypto::{encrypt_file, decrypt_file};

    
    #[test]
    fn split_and_bind_works() {
        //sanity test
        // let dir = TempDir::new("a").expect("Create temp dir").path().to_path_buf();
        let dir = PathBuf::from(".");
        let file_path = dir.join("test_secret.txt");
        
        let mut temp_file = File::create(file_path).expect("Should create temp file.");
        temp_file.write_all(b"Hello, world!").expect("Should write to temp file");
        
        let _split = split(&dir.join("test_secret.txt"), &dir, 1, 1);
        assert!(dir.join("test_secret.txt").exists());
        
        fs::remove_file(dir.join("test_secret.txt")).expect("File should be removed");
        assert!(!dir.join("test_secret.txt").exists());
        
        let _bind = bind(&dir, &dir).expect("Bind should be working");
        assert!(dir.join("test_secret.txt").exists());

        fs::remove_file(dir.join("test_secret.txt")).expect("Test cleanup");
        fs::remove_file(dir.join("test_secret_1_of_1.horcrux")).expect("Test cleanup");
    }

    #[test]
    fn matching_contents() {
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 19];
        OsRng.fill_bytes(&mut key);
        OsRng.fill_bytes(&mut nonce);

        let dir = PathBuf::from(".");

        
        

        let mut plaintext_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open(dir.join("plaintext.txt")).expect("Should create plaintext file");
        plaintext_file.write_all(b"Hello word \n").expect("Should write to temp plaintext file.");

        let mut bytes_read: Vec<u8> = Vec::new();

        plaintext_file.read_to_end(&mut bytes_read).expect("Get bytes from file");

        let mut pre_hasher = Sha256::new();
        pre_hasher.update(bytes_read);
        let plaintext_result = pre_hasher.finalize();

        let mut ciphertext_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open(dir.join("ciphertext.txt")).expect("Should create ciphertext file");
        
        let mut decrypted_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open(dir.join("decrypted.txt")).expect("Should create decrypted file");

        encrypt_file(&mut plaintext_file, &mut ciphertext_file, &key, &nonce).expect("Should encrypt contents.");
        decrypt_file(&mut ciphertext_file, &mut decrypted_file, &key, &nonce).expect("Should decrypt contents.");
        
        let mut decrypted: Vec<u8> = Vec::new();
        decrypted_file.read_to_end(&mut decrypted).expect("Get bytes from file.");
        
        let mut post_hasher = Sha256::new();
        post_hasher.update(decrypted);
        let decrypted_result = post_hasher.finalize();

        assert_eq!(plaintext_result, decrypted_result);

        fs::remove_file(dir.join("plaintext.txt")).expect("Cleanup.");
        fs::remove_file(dir.join("ciphertext.txt")).expect("Cleanup.");
        fs::remove_file(dir.join("decrypted.txt")).expect("Cleanup.");


    }

    
    #[test]
    fn splitting_a_file_permission_fails() {
        let dir = PathBuf::from(".");
        let file_path = dir.join("test_secret.txt");
        
        let mut temp_file = File::create(file_path).expect("Should create temp file.");
        temp_file.write_all(b"Hello, world!").expect("Should write to temp file");
        let split_result = split(&dir, &PathBuf::from("/root"), 1,1);
        assert!(split_result.is_err())
    }


    #[test]
    fn split_creates_dir() {
        let dir = PathBuf::from(".");
        let file_path = dir.join("test_secret.txt");
        
        let mut temp_file = File::create(file_path).expect("Should create temp file.");
        temp_file.write_all(b"Hello, world!").expect("Should write to temp file");
        
        let split_result = split(&dir.join("test_secret.txt"), &dir.join("test-folder"), 1, 1);
        assert!(dir.join("test-folder").join("test_secret_1_of_1.horcrux").exists());
        assert!(split_result.is_ok());

        fs::remove_file(dir.join("test-folder").join("test_secret_1_of_1.horcrux")).expect("Test cleanup");
        fs::remove_dir(dir.join("test-folder")).expect("Cleanup.");
    }

    // #[test]
    fn binding_malformed_file_fails() {}

    fn binding_malformed_headers_fails() {}

    #[test]
    fn binding_empty_file_fails() {
        let dir = PathBuf::from(".");
        let file_path = dir.join("test_1_of_5.horcrux");
        
        let _temp_file = File::create(file_path).expect("Should create temp file.");
        
        let bind_result = bind(&dir, &dir);
        assert!(bind_result.is_err());
        fs::remove_file(dir.join("test_1_of_5.horcrux")).expect("Cleanup")
    }

    // Add more test functions for other parts of your code
}
