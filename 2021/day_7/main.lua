#!/usr/bin/env lua

local input = (
  function ()
    local file = io.open("input.txt", "r")
    local acc = {}
    for num in string.gmatch(file:read(), '([^,]+)') do
        table.insert(acc, tonumber(num))
    end

    file:close()
    return acc
  end)()

tmp = {}
for cur = 0,math.max(table.unpack(input)) do
    acc = 0

    for _, other in ipairs(input) do
        acc = acc + math.abs(cur - other)
    end
    table.insert(tmp, acc)
end

smallest_cost = math.min(table.unpack(tmp))
print("Answer First: " .. smallest_cost)

tmp = {}
for cur = 0,math.max(table.unpack(input)) do
    acc = 0

    for _, other in ipairs(input) do
        diff = math.abs(cur - other)

        acc = acc + math.floor((diff * (diff + 1)) / 2)
    end
    table.insert(tmp, acc)
end

smallest_cost = math.min(table.unpack(tmp))
print("Answer Second: " .. smallest_cost)
