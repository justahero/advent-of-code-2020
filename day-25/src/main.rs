/// Executes the specific loop a number of times
fn run_loop(index: u64, subject: u64) -> u64 {
    (1..index).fold(subject, |value, _| (value * subject) % 20201227)
}

/// Determine the loop size of handshake
fn find_loop_size(public_key: u64, subject: u64) -> u64 {
    let mut index = 1;
    let mut value = subject;

    loop {
        value = (value * subject) % 20201227;
        index += 1;

        if value == public_key {
            return index;
        }
    }
}

/// Determine the shared encryption key
fn find_encryption_code(left_pub: u64, right_pub: u64, subject: u64) -> u64 {
    let left_loop = find_loop_size(left_pub, subject);
    let right_loop = find_loop_size(right_pub, subject);

    let left = run_loop(right_loop, left_pub);
    let right = run_loop(left_loop, right_pub);

    assert_eq!(left, right);

    left
}

fn main() {
    let card_pub = 1327981;
    let door_pub = 2822615;

    let code = find_encryption_code(card_pub, door_pub, 7);
    dbg!(code);
}

#[cfg(test)]
mod tests {
    use crate::{find_encryption_code, find_loop_size};

    #[test]
    fn test_find_loop_sizes() {
        assert_eq!(8, find_loop_size(5764801, 7));
        assert_eq!(11, find_loop_size(17807724, 7));
    }

    #[test]
    fn test_find_encryption_code() {
        assert_eq!(14897079, find_encryption_code(5764801, 17807724, 7));
    }
}
