use rand::Rng;

static BIN_SIZE: u32 = 400;
static PACKAGE_SIZE: u32 = 100;

fn main() {
    let packages = packages(PACKAGE_SIZE);
    let bins = optimize(BIN_SIZE, packages);

    println!(
        "Optimized {} packages into {} bins",
        PACKAGE_SIZE,
        bins.len()
    );
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Package {
    size: u32,
}

struct Bin {
    packages: Vec<Package>,
}

impl Bin {
    fn new() -> Self {
        Self { packages: vec![] }
    }

    fn add_package(&mut self, package: &Package) {
        self.packages.push(package.to_owned());
    }

    fn get_size(&self) -> u32 {
        self.packages.iter().fold(0, |a, b| a + b.size)
    }
}

fn packages(n: u32) -> Vec<Package> {
    let mut rng = rand::thread_rng();
    let mut packages = Vec::new();
    let mut i = 0;

    while i < n {
        packages.push(Package {
            size: rng.gen_range(0..100),
        });
        i += 1;
    }

    packages
}

/// This is the solution part of the challenge.
fn optimize(bin_size: u32, mut packages: Vec<Package>) -> Vec<Bin> {
    // Sort the packages by size.
    packages.sort_by(|a, b| b.cmp(a));

    // Fold the packages as a tuple of (current bin index, bins).
    // The goal is to fill the packages as they come into the accumulator.
    // When full, we bump up the index and move on with a new bin.
    let (_, bins) = packages.iter().fold(
        (0, vec![]),
        |(mut current_bin_index, mut bins): (usize, Vec<Bin>), package| {
            if let Some(current_bin) = bins.get_mut(current_bin_index) {
                let next_expected_size = current_bin.get_size() + package.size;

                if next_expected_size <= bin_size {
                    current_bin.add_package(package);
                } else {
                    current_bin_index += 1;

                    let mut new_bin = Bin::new();

                    new_bin.add_package(package);

                    bins.push(new_bin);
                }
            } else {
                let mut new_bin = Bin::new();

                new_bin.add_package(package);

                bins.push(new_bin);
            }

            (current_bin_index, bins)
        },
    );

    bins
}

#[cfg(test)]
mod tests {
    use crate::{optimize, packages, BIN_SIZE, PACKAGE_SIZE};

    #[test]
    fn check_optimize() {
        let packages = packages(PACKAGE_SIZE);
        let packages_copy = packages.to_owned();
        let bins = optimize(BIN_SIZE, packages);
        let packages_size = packages_copy.iter().fold(0, |a, b| a + b.size);
        let bins_size = bins.iter().fold(0, |a, b| a + b.get_size());

        assert_eq!(
            packages_size, bins_size,
            "check that the packages and the bins have the same size"
        );
        assert!(
            bins.iter().all(|bin| (bin.get_size() <= BIN_SIZE)),
            "check that all the bins have the correct size"
        );
    }
}
