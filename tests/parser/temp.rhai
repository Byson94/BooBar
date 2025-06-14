let test = "Hello!";

let custom = #{
    "test": #{ type: "label", content: "Hello from Lua!" },
    "time": #{ type: "label", content: test }
};

let windows = #{
    "bar1": #{
        left_contents: "custom.test",
        right_contents: "custom.time",
        width: 600,
        height: 20,
        win_type: "dock",
        position: "top"
    },
    "bar2": #{
        center_contents: "custom.time",
        width: 600,
        position: "top"
    }
};

//let main = #{
//    bar: [ "bar1", "bar2" ]
//};