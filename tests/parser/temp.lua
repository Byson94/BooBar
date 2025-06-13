poll("time", function() return os.date("%H:%M:%S") end, "1s")

custom = {
  test = { type = "label", content = "Hello from Lua!" },
  time = {
    type = "label",
    content = boo.time
  }
}

bars = {
  bar1 = {
    left_contents = "custom.test",
    right_contents = "custom.time",
    width = 600,
    position = "top"
  }
}

main = { bar = {"bar1"} }