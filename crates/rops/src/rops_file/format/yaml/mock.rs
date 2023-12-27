mod rops_file {
    use std::{fmt::Display, str::FromStr};

    use crate::*;

    impl<S: RopsFileState> MockFileFormatUtil<YamlFileFormat> for RopsFile<S, YamlFileFormat>
    where
        RopsFileFormatMap<S::MapState, YamlFileFormat>: MockFileFormatUtil<YamlFileFormat>,
        RopsFileMetadata<S::MetadataState>: MockFileFormatUtil<YamlFileFormat>,
        <<S::MetadataState as RopsMetadataState>::Mac as FromStr>::Err: Display,
    {
        fn mock_format_display() -> String {
            indoc::formatdoc! {"
                {}
                sops:
                {}",
                RopsFileFormatMap::mock_format_display(),
                textwrap::indent(&RopsFileMetadata::mock_format_display(),"  ")
            }
        }
    }
}

mod map {
    use crate::*;

    impl MockFileFormatUtil<YamlFileFormat> for RopsFileFormatMap<DecryptedMap, YamlFileFormat> {
        fn mock_format_display() -> String {
            indoc::indoc! {"
                hello: world!
                nested_map:
                  null_key: null
                  array:
                  - string
                  - nested_map_in_array:
                      integer: 1234
                  - float: 1234.56789
                booleans:
                - true
                - false
                escape_unencrypted: plaintext"
            }
            .to_string()
        }
    }

    #[cfg(feature = "aes-gcm")]
    impl MockFileFormatUtil<YamlFileFormat> for RopsFileFormatMap<EncryptedMap<AES256GCM>, YamlFileFormat> {
        fn mock_format_display() -> String {
            indoc::indoc! {"
                hello: ENC[AES256_GCM,data:3S1E9am/,iv:WUQoQTrRXw/tUgwpmSG69xWtd5dVMfe8qUly1VB8ucM=,tag:nQUDkuh0OR1cjR5hGC5jOw==,type:str]
                nested_map:
                  null_key: null
                  array:
                  - ENC[AES256_GCM,data:ANbeNrGp,iv:PRWGCPdOttPr5dlzT9te7WWCZ90J7+CvfY1vp60aADM=,tag:PvSLx4pLT5zRKOU0df8Xlg==,type:str]
                  - nested_map_in_array:
                      integer: ENC[AES256_GCM,data:qTW5qw==,iv:ugMxvR8YPwDgn2MbBpDX0lpCqzJY3GerhbA5jEKUbwE=,tag:d8utfA76C4XPzJyDfgE4Pw==,type:int]
                  - float: ENC[AES256_GCM,data:/MTg0fCennyN8g==,iv:+/8+Ljm+cls7BbDYZnlg6NVFkrkw4GkEfWU2aGW57qE=,tag:26uMp2JmVAckySIaL2BLCg==,type:float]
                booleans:
                - ENC[AES256_GCM,data:bCdz2A==,iv:8kD+h1jClyVHBj9o2WZuAkjk+uD6A2lgNpcGljpQEhk=,tag:u3/fktl5HfFrVLERVvLRGw==,type:bool]
                - ENC[AES256_GCM,data:SgBh7wY=,iv:0s9Q9pQWbsZm2yHsmFalCzX0IqNb6ZqeY6QQYCWc+qU=,tag:OZb76BWCKbDLbcil4c8fYA==,type:bool]
                escape_unencrypted: plaintext"
            }
            .to_string()
        }
    }
}

mod metadata {
    mod core {
        use std::{fmt::Display, str::FromStr};

        use crate::*;

        impl<I: IntegrationTestUtils> MockFileFormatUtil<YamlFileFormat> for IntegrationMetadataUnit<I>
        where
            I::Config: MockFileFormatUtil<YamlFileFormat>,
            for<'a> &'a I::PublicKey: From<&'a I::Config>,
        {
            fn mock_format_display() -> String {
                indoc::formatdoc! {"
                    {}
                    enc: |
                    {}",
                    I::Config::mock_format_display(), textwrap::indent(I::mock_encrypted_data_key_str(), "  ")
                }
            }
        }

        impl<S: RopsMetadataState> MockFileFormatUtil<YamlFileFormat> for RopsFileMetadata<S>
        where
            S::Mac: MockDisplayTestUtil,
            <S::Mac as FromStr>::Err: Display,
        {
            fn mock_format_display() -> String {
                let mut metadata_string = String::new();

                #[cfg(feature = "age")]
                {
                    metadata_string.push_str(&display_integration_metadata_unit::<AgeIntegration>());
                }

                metadata_string.push_str(&indoc::formatdoc! {"
                    lastmodified: {}
                    mac: {}
                    unencrypted_suffix: {}
                    ",
                    LastModifiedDateTime::mock_display(),
                    S::Mac::mock_display(),
                    PartialEncryptionConfig::mock_display()
                });

                return metadata_string;

                fn display_integration_metadata_unit<I: IntegrationTestUtils>() -> String
                where
                    IntegrationMetadataUnit<I>: MockFileFormatUtil<YamlFileFormat>,
                    for<'a> &'a I::PublicKey: From<&'a I::Config>,
                {
                    let integration_metadata = IntegrationMetadataUnit::<I>::mock_format_display();
                    let (first_metadata_line, remaning_metata_lines) = integration_metadata
                        .split_once('\n')
                        .expect("no newline delimeter in integration metadata");

                    indoc::formatdoc! {"
                        {}:
                        - {}
                        {}",
                        I::NAME,
                        first_metadata_line,
                        textwrap::indent(remaning_metata_lines, "  ")
                    }
                }
            }
        }
    }

    #[cfg(feature = "age")]
    mod age {
        use crate::*;

        impl MockFileFormatUtil<YamlFileFormat> for AgeConfig {
            fn mock_format_display() -> String {
                format!("recipient: {}", AgeIntegration::mock_public_key_str(),)
            }
        }
    }
}
