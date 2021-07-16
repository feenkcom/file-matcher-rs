import hudson.tasks.test.AbstractTestResultAction
import hudson.model.Actionable
import hudson.tasks.junit.CaseResult

pipeline {
    agent none
    parameters { booleanParam(name: 'PUBLISH', defaultValue: false, description: 'Set to true to publish a new version to crates.io') }
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        disableConcurrentBuilds()
    }
    environment {
        GITHUB_TOKEN = credentials('githubrelease')
        AWSIP = 'ec2-18-197-145-81.eu-central-1.compute.amazonaws.com'

        MACOS_INTEL_TARGET = 'x86_64-apple-darwin'
        MACOS_M1_TARGET = 'aarch64-apple-darwin'
        WINDOWS_AMD64_TARGET = 'x86_64-pc-windows-msvc'
        LINUX_AMD64_TARGET = 'x86_64-unknown-linux-gnu'
    }

    stages {
        stage ('Parallel build') {
            parallel {
                stage ('MacOS x86_64') {
                    agent {
                        label "${MACOS_INTEL_TARGET}"
                    }
                    environment {
                        TARGET = "${MACOS_INTEL_TARGET}"
                        PATH = "$HOME/.cargo/bin:/usr/local/bin/:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --release --all-features"
                        sh "cargo test --all-features"
                        sh "cargo clippy --manifest-path Cargo.toml -- -W clippy::style -W clippy::correctness -W clippy::complexity -W clippy::perf"
                    }
                }
                stage ('MacOS M1') {
                    agent {
                        label "${MACOS_M1_TARGET}"
                    }

                    environment {
                        TARGET = "${MACOS_M1_TARGET}"
                        PATH = "$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --release --all-features"
                        sh "cargo test --all-features"
                        sh "cargo clippy --manifest-path Cargo.toml -- -W clippy::style -W clippy::correctness -W clippy::complexity -W clippy::perf"
                    }
                }
                stage ('Linux x86_64') {
                    agent {
                        label "${LINUX_AMD64_TARGET}"
                    }
                    environment {
                        TARGET = "${LINUX_AMD64_TARGET}"
                        PATH = "$HOME/.cargo/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --release --all-features"
                        sh "cargo test --all-features"
                        sh "cargo clippy --manifest-path Cargo.toml -- -W clippy::style -W clippy::correctness -W clippy::complexity -W clippy::perf"
                    }
                }
                stage ('Windows x86_64') {
                    agent {
                        label "${WINDOWS_AMD64_TARGET}"
                    }

                    environment {
                        TARGET = "${WINDOWS_AMD64_TARGET}"
                        LLVM_HOME = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC\\Tools\\Llvm\\x64'
                        LIBCLANG_PATH = "${LLVM_HOME}\\bin"
                        CMAKE_PATH = 'C:\\Program Files\\CMake\\bin'
                        MSBUILD_PATH = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\MSBuild\\Current\\Bin'
                        CARGO_HOME = "C:\\.cargo"
                        CARGO_PATH = "${CARGO_HOME}\\bin"
                        PATH = "${CARGO_PATH};${LIBCLANG_PATH};${MSBUILD_PATH};${CMAKE_PATH};$PATH"
                    }

                    steps {
                        powershell 'git clean -fdx'
                        powershell "cargo build --release --all-features"
                        powershell "cargo test --all-features"
                        powershell "cargo clippy --manifest-path Cargo.toml -- -W clippy::style -W clippy::correctness -W clippy::complexity -W clippy::perf"
                    }
                }
            }
        }
        stage ('Release') {
            agent {
                label "${LINUX_AMD64_TARGET}"
            }
            environment {
                TARGET = "${LINUX_AMD64_TARGET}"
            }
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main') && !params.PUBLISH
                }
            }
            steps {
                sh "wget -O feenk-releaser https://github.com/feenkcom/releaser-rs/releases/latest/download/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"

                sh """
                ./feenk-releaser \
                    --owner feenkcom \
                    --repo file-matcher-rs \
                    --token GITHUB_TOKEN \
                    --bump-patch \
                    --auto-accept """
            }
        }
        stage ('Publish') {
            agent {
                label "${LINUX_AMD64_TARGET}"
            }
            environment {
                TARGET = "${LINUX_AMD64_TARGET}"
            }
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main') && params.PUBLISH
                }
            }
            steps {
                sh "cargo publish --dry-run"
                sh "cargo publish"

                sh "wget -O feenk-releaser https://github.com/feenkcom/releaser-rs/releases/latest/download/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"

                sh """
                ./feenk-releaser \
                    --owner feenkcom \
                    --repo file-matcher-rs \
                    --token GITHUB_TOKEN \
                    --bump-minor \
                    --auto-accept """
            }
        }
    }
}
