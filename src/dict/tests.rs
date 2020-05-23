use super::*;

#[test]
fn test_add_dict() {
    let mut dict = Dict::new();
    dict.add("งาน");
    dict.add("งานบ้าน");
    dict.add("งานกลุ่ม");
    dict.add("งานเรือน");
    dict.add("การงาน");
    dict.add("การบ้าน");
    dict.add("งาช้าง");
    assert_eq! {
        dict,
        Dict {
            root: vec![
                Node {
                    childs: Some(vec![
                        Node {
                            childs: Some(vec![]),
                            terminal: true,
                            value: "งาน".to_owned()
                        },
                        Node {
                            childs: Some(vec![]),
                            terminal: true,
                            value: "บ้าน".to_owned()
                        },
                    ]),
                    terminal: false,
                    value: "การ".to_owned()
                },
                Node {
                    childs: Some(
                        vec![
                            Node {
                                childs: Some(
                                    vec![],
                                ),
                                terminal: true,
                                value: "ช้าง".to_owned(),
                            },
                            Node {
                                childs: Some(
                                    vec![
                                        Node {
                                            childs: Some(
                                                vec![],
                                            ),
                                            terminal: true,
                                            value: "กลุ่ม".to_owned(),
                                        },
                                        Node {
                                            childs: Some(
                                                vec![],
                                            ),
                                            terminal: true,
                                            value: "บ้าน".to_owned(),
                                        },
                                        Node {
                                            childs: Some(
                                                vec![],
                                            ),
                                            terminal: true,
                                            value: "เรือน".to_owned(),
                                        },],
                                ),
                                terminal: true,
                                value: "น".to_owned(),
                            },
                        ],
                    ),
                    terminal: false,
                    value: "งา".to_owned(),
                },
            ]
        }
    };
}

#[test]
fn load_dict() {
    let dict = Dict::load_txt("data/th.txt").unwrap();
    let manual_add = Dict { 
        root: vec! [
            Node { 
                childs: Some(vec![
                    Node { 
                        childs: Some(vec![
                            Node { 
                                childs: Some(vec![]), 
                                terminal: true, 
                                value: "ณ์".to_owned()
                            }, Node { 
                                childs: Some(vec![
                                    Node { 
                                        childs: Some(vec![
                                            Node { 
                                                childs: Some(vec![]), 
                                                terminal: true, 
                                                value: "ร".to_owned()
                                            }, 
                                            Node { 
                                                childs: Some(vec![]), 
                                                terminal: true, 
                                                value: "าร".to_owned()
                                            }
                                        ]), 
                                        terminal: false, 
                                        value: "ก".to_owned()
                                    }
                                ]), 
                                terminal: true, 
                                value: "รม".to_owned()
                            }
                        ]), 
                        terminal: false, 
                        value: "ร".to_owned()
                    }, 
                    Node { 
                        childs: Some(vec![
                            Node { 
                                childs: Some(vec![]), 
                                terminal: true, 
                                value: "กระจัด".to_owned()
                            }, 
                            Node { 
                                childs: Some(vec![]), 
                                terminal: true, 
                                value: "งาน".to_owned()
                            }, 
                            Node { 
                                childs: Some(vec![
                                    Node { 
                                        childs: Some(vec![]), 
                                        terminal: true, 
                                        value: "ูรณ์".to_owned()
                                    }, Node { 
                                        childs: Some(vec![]), 
                                        terminal: true, 
                                        value: "้าน".to_owned()
                                    }
                                ]), 
                                terminal: false, 
                                value: "บ".to_owned()
                            }, 
                            Node { 
                                childs: Some(vec![]), 
                                terminal: true, 
                                value: "ละเล่น".to_owned()
                            }
                        ]), 
                        terminal: false, 
                        value: "าร".to_owned()
                    }
                ]), 
                terminal: false, 
                value: "ก".to_owned()
            }, 
            Node { 
                childs: Some(vec![]), 
                terminal: true, 
                value: "อาจารย์".to_owned()
            }, 
            Node { 
                childs: Some(vec![
                    Node { 
                        childs: Some(vec![]), 
                        terminal: true, 
                        value: "การเอางาน".to_owned()
                    }
                ]), 
                terminal: true, 
                value: "เอา".to_owned()
            }
        ]
    };
    assert_eq!(dict, manual_add);
}

#[test]
fn test_sized_dict() {
    let dict = Dict::load_txt("data/th.txt").unwrap();
    let dict: SizedDict = dict.into();

    assert_eq!(
        dict,
        SizedDict { 
            root: Box::new([
                SizedNode { 
                    childs: Box::new([
                        SizedNode { 
                            childs: Box::new([
                                SizedNode { 
                                    childs: Box::new([]), 
                                    terminal: true, 
                                    value: "ณ์".to_owned()
                                }, SizedNode { 
                                    childs: Box::new([
                                        SizedNode { 
                                            childs: Box::new([
                                                SizedNode { 
                                                    childs: Box::new([]), 
                                                    terminal: true, 
                                                    value: "ร".to_owned()
                                                }, 
                                                SizedNode { 
                                                    childs: Box::new([]), 
                                                    terminal: true, 
                                                    value: "าร".to_owned()
                                                }
                                            ]), 
                                            terminal: false, 
                                            value: "ก".to_owned()
                                        }
                                    ]), 
                                    terminal: true, 
                                    value: "รม".to_owned()
                                }
                            ]), 
                            terminal: false, 
                            value: "ร".to_owned()
                        }, 
                        SizedNode { 
                            childs: Box::new([
                                SizedNode { 
                                    childs: Box::new([]), 
                                    terminal: true, 
                                    value: "กระจัด".to_owned()
                                }, 
                                SizedNode { 
                                    childs: Box::new([]), 
                                    terminal: true, 
                                    value: "งาน".to_owned()
                                }, 
                                SizedNode { 
                                    childs: Box::new([
                                        SizedNode { 
                                            childs: Box::new([]), 
                                            terminal: true, 
                                            value: "ูรณ์".to_owned()
                                        }, SizedNode { 
                                            childs: Box::new([]), 
                                            terminal: true, 
                                            value: "้าน".to_owned()
                                        }
                                    ]), 
                                    terminal: false, 
                                    value: "บ".to_owned()
                                }, 
                                SizedNode { 
                                    childs: Box::new([]), 
                                    terminal: true, 
                                    value: "ละเล่น".to_owned()
                                }
                            ]), 
                            terminal: false, 
                            value: "าร".to_owned()
                        }
                    ]), 
                    terminal: false, 
                    value: "ก".to_owned()
                }, 
                SizedNode { 
                    childs: Box::new([]), 
                    terminal: true, 
                    value: "อาจารย์".to_owned()
                }, 
                SizedNode { 
                    childs: Box::new([
                        SizedNode { 
                            childs: Box::new([]), 
                            terminal: true, 
                            value: "การเอางาน".to_owned()
                        }
                    ]), 
                    terminal: true, 
                    value: "เอา".to_owned()
                }
            ])
        }
    );
}