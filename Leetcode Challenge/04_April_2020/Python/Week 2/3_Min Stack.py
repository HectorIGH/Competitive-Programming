#Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
#
#push(x) -- Push element x onto stack.
#pop() -- Removes the element on top of the stack.
#top() -- Get the top element.
#getMin() -- Retrieve the minimum element in the stack.
# 
#
#Example:
#
#MinStack minStack = new MinStack();
#minStack.push(-2);
#minStack.push(0);
#minStack.push(-3);
#minStack.getMin();   --> Returns -3.
#minStack.pop();
#minStack.top();      --> Returns 0.
#minStack.getMin();   --> Returns -2.

class MinStack:

    def __init__(self):
        """
        initialize your data structure here.
        """
        self.stack = []
        self.minimo = 1e9 - 5
        

    def push(self, x: int) -> None:
        self.minimo = min(self.minimo, x) if self.stack else x
        self.stack.append(x)
        

    def pop(self) -> None:
        self.stack.pop()
        self.minimo = min(self.stack) if self.stack else 1e9 - 5
        

    def top(self) -> int:
        return self.stack[-1]
        

    def getMin(self) -> int:
        return self.minimo
        #return min(self.stack)


# Your MinStack object will be instantiated and called as such:
#obj = MinStack()
#obj.push(-2)
#obj.push(0)
#obj.push(-3)
#print(obj.getMin())
#obj.pop()
#print(obj.top())
#print(obj.getMin())

obj = MinStack()

obj.push(2147483646)
obj.push(2147483646)
obj.push(2147483647)
print(obj.top())
obj.pop()
print(obj.getMin())
obj.pop()
print(obj.getMin())
obj.pop()
obj.push(2147483647)
print(obj.top())
print(obj.getMin())
obj.push(-2147483648)
print(obj.top())
print(obj.getMin())
obj.pop()
print(obj.getMin())