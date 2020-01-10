# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None
import ll

class Solution(object):
    def hasCycle(self, head):
        """
        :type head: ListNode
        :rtype: bool
        """
        slow, fast = head;
        while slow != None:
            slow = slow.next
            fast = fast.next.next

            if slow == fast:
                return True

        return False
        
        
def main():
    

main()
