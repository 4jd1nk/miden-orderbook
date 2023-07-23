pub fn get_masm_code() -> String {{
    format!(
"#
# Library for Red-Black Binary Search Tree implementation of orderbook management
#
# Reference: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
#

proc.getNodePointer
  # Inputs:  [creation_counter, ...]
  # Outputs: [node_pointer, ...]
  # node_pointer = creation_counter * 3 + 1
  #
  # Comments: node_pointer >= 1.  Three items in memory: coordinate_word, order_word, and change flag
  #
  push.3 mul push.1 add  # Generate node_pointer
end

proc.getTotalNodes
  mem_load.0 push.2 sub 
end

proc.SaveCoordinate
  # Inputs: [node_pointer, coordinate_word, ...]
  # Output: [...]
  #
  # Comments: saves coordinate word to address node_pointer, 
  #
  mem_storew dropw # Save in RAM          [...]
end

proc.SaveOrder
  # Inputs: [node_pointer, order_word, ...]
  # Output: [...]
  #
  # Comments: saves order word to address node_pointer+1, 
  #
  push.1 add       # Set memory address   [node_pointer+1, order_word, ...]
  mem_storew dropw # Save in RAM          [...]
end

proc.SaveChangeFlag
  # Inputs: [node_pointer, ...]
  # Output: [...]
  #
  # Comments: saves order word to address node_pointer+1, 
  #
  push.1 swap      # Set change flag      [node_pointer, 1, ...]
  push.2 add       # Set memory address   [node_pointer+2, 1, ...]
  mem_store        # Save in RAM          [...]
end

proc.UpdateCoordinate
  # Inputs: [node_pointer, coordinate_word, ...]
  # Output: [...]
  #
  # Comments: saves coordinate word to address node_pointer, 
  #            Change flag is updated
  #
  dup exec.SaveChangeFlag
  exec.SaveCoordinate
end

proc.UpdateOrder
  # Inputs: [node_pointer, order_word, ...]
  # Output: [...]
  #
  # Comments: saves order word to address node_pointer+1. 
  #            Change flag is updated
  #
  dup exec.SaveChangeFlag
  exec.SaveOrder
end

proc.SaveNode
  # Inputs: [node_pointer, coordinate_word, order_word]
  # Output: saves coordinate word to address node_pointer, 
  # and order_word to address node_pointer+1
  #
  dup movdn.5 
  exec.SaveCoordinate
  exec.SaveOrder
end

proc.UpdateNode
  # Inputs: [node_pointer, coordinate_word, order_word]
  # Output: saves coordinate word to address node_pointer, 
  # and order_word to address node_pointer+1
  #
  dup movdn.5 
  exec.UpdateCoordinate
  exec.UpdateOrder
end

proc.InitTree
  # Inputs:  [...]
  # Outputs: [...]
  #
  # Comments: Initialized rbBST tree by creating a special first node
  # With color and parent pointer set to special value of 2
  # 
  push.0.0.0.0.0.0.0.2                         # [2,0,0,0,0,0,0,0, ...]
  push.0 exec.getNodePointer exec.SaveNode     
end

proc.getCoordinates
  # Inputs:  [node_pointer, ...]
  # Outputs: [color, Parent_pointer, L_child_pointer, R_child_pointer, ...]
  # 
  push.0.0.0.0 
  movup.4 mem_loadw             # Load node Coordinates from memory
end

proc.getColorFromCoordinates
  # Inputs:  [color, Parent_pointer, L_child_pointer, R_child_pointer, ...]
  # Outputs: [color, ...]
  # 
  movdn.3         # Move color to bottom of current word
  drop drop drop  # Drop unnecessary pointer data
end

proc.getChildrenFromCoordinates
  # Inputs:  [color, Parent_pointer, L_child_pointer, R_child_pointer, ...]
  # Outputs: [L_child_pointer, R_child_pointer, ...]
  # 
  drop drop
end

proc.getOrder
  # Inputs:  [node_pointer, ...]
  # Outputs: [quantity, price, time, orderID, ...]
  # 
  push.0.0.0.0 movup.4 push.1 add mem_loadw  # Load node Order from memory
end

proc.getChangeFlag
  # Inputs:  [node_pointer, ...]
  # Outputs: [change_flag, ...]
  #
  push.2 add mem_load
end

proc.LoadNode
  # Inputs: [node_pointer, ...]
  # Output: [coordinate_word, order_word]
  #
  dup exec.getOrder
  movup.4
  dup exec.getCoordinates
  movup.4 
  exec.getChangeFlag
end

proc.PrintAllNodesEssential
  # Inputs: [...]
  # Output: [N, coordinate_word_N, price_N, ..., 2, coordinate_word_2, price_2, 1, coordinate_word_3, price_3, ... ]
  #
  push.0 dup                            # [0,0,....]
  mem_load.0 lt                         # [isLessThanCreationNumber_bool, 0, ...]
  while.true
     push.1 add dup                     # [j++, j++, ...]
     exec.getNodePointer                # [node_pointer, j++, ...]
     dup exec.getOrder                  # [order_word, node_pointer, j++, ...]
     drop swap drop swap drop           # [order_price, node_pointer, j++, ...]
     swap exec.getCoordinates           # [coordinate_word, order_price, j++, ...]
     movup.5 dup                        # [j++, j++, coordinate_word, order_price, j++, ...]
     exec.getNodePointer swap           # [j++, node_pointer, coordinate_word, order_price, j++, ...]
     dup mem_load.0 lt                  # [moreNodes_bool, j++, node_pointer, coordinate_word, order_price, j++, ...]
  end
  drop
end

proc.PrintAllNodes
  # Inputs: [...]
  # Output: [N, coordinate_word_N, order_word_N, ..., 2, coordinate_word_2, order_word_2, 1, coordinate_word_3, order_word_3, ... ]
  #
  push.0 push.1                            # [1,0,....]
  while.true
     dup                                # [j, j, ...]
     exec.getNodePointer exec.LoadNode  # [node_change_flag, node_coordinate_word, node_order_word, j, ...]
     drop movup.8 dup                   # [j, j, node_coordinate_word, node_order_word, ...]
     exec.getNodePointer swap           # [j, node_pointer, node_coordinate_word, node_order_word, ...]
     push.1 add                         # [j++, node_pointer, node_coordinate_word, node_order_word, ...]
     dup mem_load.0 lt                  # [moreNodes_bool, j++, node_pointer, node_coordinate_word, node_order_word, ...]
  end
  drop
end

proc.PrintChangeLog
  # Inputs: [...]
  # Output: [N, coordinate_word_N, order_word_N, ..., 2, coordinate_word_2, order_word_2, 1, coordinate_word_3, order_word_3, ... ]
  #
  # Comments: Prints all node data only for orders whose change flag has changed
  #
  push.0 dup                           # [0,0,....]
  mem_load.0 lt                        # [isLessThanCreationNumber_bool, 0, ...]
  while.true
    push.1 add dup                     # [j++, j++, ...]
    exec.getNodePointer                # [node_pointer, j++, ...] 
    dup exec.getChangeFlag             # [node_change_flag, node_pointer, j++, ...]
    if.true 
      exec.LoadNode drop               # [node_coordinate_word, node_order_word, j++, ...]
      movup.8 dup                      # [j++, j++, node_coordinate_word, node_order_word, ...]
      exec.getNodePointer swap         # [j++, node_pointer, node_coordinate_word, node_order_word, ...]
    else 
      drop
    end                
    dup mem_load.0 lt                  # [moreNodes_bool, j++, node_pointer, node_coordinate_word, node_order_word, ...]
  end
  drop
end

proc.CompareOrders
  # Input:   [Order_word_A, Order_Word_B, ...]
  # Outputs: [compare_bool, ...]
  #
  # Comment: compare_bool = 0 (1) if Order_A < Order B (Order_A > Order B)
  #
  # Order_A > Order_B if ( (price_A > price_B) or ((price_A==price_B) and (time_A > time_B)) )
  #
  ### COMPARE PRICES ###
  dup.5                      # Duplicate Price_B  
  dup.2                      # Duplicate Price_A
  lt                         # Check Price_B < Price_A ?
  if.true                    # Price_B < Price_A
    dropw dropw push.1       # Push 1 to signal result Order_A > Order_B
  else
    dup.5 dup.2 neq          # Duplicate Price_B, Price_A, and check for equality
    if.true                  # Price_A < Price_B 
      dropw dropw push.0     # Push 0 to signal result Order_B > Order_A
    else                     # Price_A = Price_B
      dup.6 dup.3            # Duplicate Time_B and Time_A 
      lt                     # Check Time_B < Time_A ?
      if.true                # Time_B < Time_A
        dropw dropw push.1   # Push 1 to signal result Order_A > Order_B
      else                   # Time_B > Time_A
        dropw dropw push.0   # Push 0 to signal result Order_A < Order_B
      end 
    end 
  end 
end

proc.setLeftChildToParent
  # Inputs:  [Parent_pointer, New_Child_pointer, ...]
  # Outputs: [...]
  #
  # Comments: Whithin P_coordinates: L_child_pointer -> New_Child_pointer
  #
  dup exec.getCoordinates    # Duplicate P_pointer and load coordinates     [___, ___, Lchild_pointer, ___, parent_pointer, new_Lchild_pointer, ...]
  movup.5                    # Move new Lchild to the top of the stack      [new_Lchild_pointer, ___, ___, Lchild_pointer, ___, parent_pointer, ...]
  swap.3 drop                # Swap with old L_child pointer                [ ___, ___, new_Lchild_pointer, ___, parent_pointer, ...]       
  movup.4                    # Move P_pointer to the top of the stack       [parent_pointer,  ___, ___, new_Lchild_pointer, ___, ...]
  dup exec.SaveChangeFlag    # Save change flag                             [parent_pointer,  ___, ___, new_Lchild_pointer, ___, ...]
  exec.SaveCoordinate        # Save updated Parent coordinates              [...]
end

proc.setRightChildToParent
  # Inputs:  [Parent_pointer, New_Child_pointer, ...]
  # Outputs: [...]
  #
  # Comments: Whithin P_coordinates: L_child_pointer -> New_Child_pointer
  #
  dup exec.getCoordinates    # Duplicate P_pointer and load coordinates     [___, ___, ___, Rchild_pointer, parent_pointer, new_Lchild_pointer, ...]
  movup.5                    # Move new Rchild to the top of the stack      [new_child_pointer, ___, ___, ___, Rchild_pointer, parent_pointer, ...]
  swap.4 drop                # Swap with old Rchild pointer                 [___, ___, ___, new_child_pointer, parent_pointer, ...]       
  movup.4                    # Move P_pointer to the top of the stack       [parent_pointer,  ___, ___, ___, new_child_pointer, ...]
  dup exec.SaveChangeFlag    # Save change flag                             [parent_pointer,  ___, ___, ___, new_child_pointer, ...]
  exec.SaveCoordinate        # Save updated Parent coordinates              [...]
end

proc.UpdateChildInParent
  # Inputs: [Parent_pointer, Child_Pointer, ...]
  # Output: [...]
  #
  # Comments: Checks if child is L or R and updates the corresponding pointer
  #
  dup exec.SaveChangeFlag              # Save change flag              [Parent_pointer, Child_Pointer, ...]
  dup.1 exec.getOrder                  # Load child order              [child_order_word, Parent_pointer, Child_Pointer, ...]
  dup.4 exec.getOrder                  # Load parent order             [parent_order_word, child_order_word, Parent_pointer, Child_Pointer, ...]
  exec.CompareOrders                   # Compare Parent and Child      [isParentGreaterThanChild_bool, Parent_pointer, Child_Pointer, ...]
  if.true                              
    exec.setLeftChildToParent 
  else
    exec.setRightChildToParent
  end
end

proc.UpdateParentInChild
  # Inputs: [Parent_pointer, Child_Pointer, ...]
  # Output: [ ...]
  #
  # Comments:  Sets Parent_pointer as the new parent of Child 
  #
  dup.1 exec.SaveChangeFlag # Save change flag
  dup.1 exec.getCoordinates # Load Child coordinates                   [           ____, OLD_parent_pointer,               ____, ____, Parent_pointer, Child_Pointer, ...]
  movup.4                   # Move Parent_pointer to top of the stack  [ Parent_pointer,               ____, OLD_parent_pointer, ____,           ____, Child_Pointer, ...]
  swap.2 drop               # Update parent pointer                    [           ____,     Parent_pointer,               ____, ____,  Child_Pointer, ...]
  movup.4                   # Move Child_pointer to top of the stack   [  Child_Pointer,               ____,     Parent_pointer, ____,           ____, ...]
  exec.SaveCoordinate       # Save updated coordinates                 [ ...]  
end

proc.HasChildren
  # Inputs:  [ node_pointer, ...]
  # Outputs: [hasChildren_bool, ...]
  #
  # Comments: Returns terminal_bool=1 if node has children.
  # terminal_bool=0 otherwise
  #
  exec.getCoordinates
  exec.getChildrenFromCoordinates
  push.0 eq 
  if.true
    push.0 eq 
    if.true
      push.0
    else
      push.1
    end 
  else
    drop push.1
  end
end

proc.getRootPointer 
  push.0 exec.getNodePointer exec.getCoordinates exec.getChildrenFromCoordinates
  dup push.0 eq 
  if.true 
    drop 
  else 
    swap drop
  end
end

proc.FindParent
  # Inputs:  [new_order_word, ...]
  # Outputs: [parent_node_pointer, new_order_wordA, ...]
  #
  exec.getRootPointer loc_store.3           # Initialize Temp Pointer To Tree Root   [new_order_word, ...]
  push.1                                  # Enter while loop                       [1, new_order_word, ...]
  while.true
    ### COMPARE ###
    dupw                      # Duplicate new order word                           [new_order_word, new_order_word, ...]
    loc_load.3 exec.getOrder  # Load Temp Parent Order                             [temp_order_word, new_order_word, new_order_word, ...]
    exec.CompareOrders        # Compare temp to new (returns 1 if parent>new)      [isTempOrderGreater_bool, new_order_word, ...]

    ### LOAD TEMP's CHILDREN ###
    loc_load.3 exec.getCoordinates       # Get coordinates of temp order           [temp_coordinate_word, isTempOrderGreater_bool, new_order_word, ...]
    exec.getChildrenFromCoordinates      # Get temp order child pointers           [temp_Lchild_pointer, temp_Rchild_pointer, isTempOrderGreater_bool, new_order_word, ...]

    movup.2                   # Get order compare back on top of stack             [isTempOrderGreater_bool, temp_Lchild_pointer, temp_Rchild_pointer, new_order_word, ...]
    if.true
      swap drop               # Isolate Left child pointer address                 [temp_Lchild_pointer, new_order_word, ...]
    else
      drop                   # Isolate Right child pointer address                 [temp_Rchild_pointer, new_order_word, ...]
    end

    dup push.0 neq            # Check if child is not NIL                          [isChildNotNIL_bool, temp_Xchild_pointer, new_order_word, ...]
    if.true
      loc_store.3             # Update temp parent pointer                         [new_order_word, ...]
      push.1                  # Continue while loop                                [1, new_order_word, ...]
    else
      drop push.0             # Child slot is empty. Insert node here              [0, new_order_word, ...]
    end
  end
  loc_load.3                  # Return proper parent in tree                       [temp_parent_pointer, new_order_word, ...]
end

proc.getParentPointer
  # Inputs:  [node_pointer, ...]
  # Outputs: [parent_pointer, ...]
  #
  # Comments: Gets parent pointer
  # 
  exec.getCoordinates          # Loads node coordinates       [___, parent_pointer, ___, ___, ...]
  drop swap drop swap drop     # Isolates Parent pointer      [parent_pointer, ...]
end

proc.getUnclePointer
  # Inputs:  [node_pointer, ...]
  # Outputs: [uncle_pointer, ...]
  #
  # Comments: Gets uncle pointer
  #
  exec.getParentPointer dup         # Retrieves parent pointer and dups  [     parent_pointer, parent_pointer, ...]
  exec.getParentPointer             # Retrieves pointer to grandparent   [grandparent_pointer, ...]
  exec.getCoordinates               # Loads grandparent coordinates      [____, ____, relatedA_pointer, relatedB_pointer, parent_pointer, ...]
  drop drop                         # Drop unnecessary fields            [relatedA_pointer, relatedB_pointer, parent_pointer, ...]
  movup.2 dup.1 eq                  # Check if relatedA=parent           [isAparent_bool, relatedA_pointer, relatedB_pointer, ...] 
  if.true
    drop                            # RelatedB is uncle              
  else
    swap drop                       # RelatedA is uncle
  end                               # [uncle_pointer,  ...]
end

proc.isNIL
  # Inputs:  [node_pointer, ...]
  # Outputs: [isNIL_bool, ...]
  #
  # Comments: Gets uncle pointer
  #  
  exec.getOrder                        # Load Order                       [order_quantity, order_price, order_time, order_index, ...] 
  push.0 eq                            # Is quantity zero                 [isQuantityZero_bool, order_price, order_time, order_index, ...]
  swap push.0 eq and                   # Are price&quantity zero          [isPriceQuantityZero_bool, order_time, order_index, ...]
  swap push.0 eq and                   # Are price&quantity&time zero     [isTimePriceQuantityZero_bool, order_index, ...]
  swap push.0 eq and                   # Are price&quantity&time&idx zero [isIdxTimePriceQuantityZero_bool, ...]
end

proc.getColor
  # Inputs:  [node_pointer, ...]
  # Outputs: [node_color, ...]
  #
  # Comments: Returns color of node at node_pointer
  #
  exec.getCoordinates exec.getColorFromCoordinates
end

proc.setColor
  # Inputs: [node_pointer, color_bool, ...]
  # Output: [...]
  #
  # Comments: Sets color of node addressed by node_pointer
  #           to the color specified by color_bool
  #
  dup exec.SaveChangeFlag    # Save change flag
  dup exec.getCoordinates    # Load node_pointer coordinates                   [  node_color,        ___, ___, ___, node_pointer, color_bool, ...]
  movup.5                    # Move color_bool to top of stack                 [color_bool, node_color, ___, ___, ___, node_pointer, ...]
  swap drop movup.4          # Swap colors and move node_pointer top of stack  [node_pointer, color_bool, ___, ___, ___, ...]
  exec.SaveCoordinate
end

proc.isRoot
  # Inputs:  [node_pointer, ...]
  # Outputs: [isRoot_bool, ...]
  #
  # Comments: Checks if node is root of the tree
  #
  exec.getParentPointer exec.getColor    # Get parent color      [parent_color, ...]
  push.2 eq                              # Is Color Init value 2 [isColorInit_bool,...]
end

proc.isUncleNIL
  # Inputs:  [node_pointer, ...]
  # Outputs: [isUncleNIL_bool, ...]
  #
  # Comments: Checks if Uncle of node is NIL
  #
  exec.getUnclePointer exec.isNIL
end

proc.isLeftChild
  # Inputs:  [node_pointer, ...]
  # Outputs: [isLeftChild_bool]
  #
  # Comments: Checks if node is a Left child of its parent
  #
  dup exec.isRoot not            # Confirm node isn't root   [isRoot_bool, node_pointer, ...]
  if.true
    dup exec.getParentPointer    # Loads pointer to parent   [parent_pointer, node_pointer, ...]
    exec.getCoordinates          # Loads parent coordiantes  [___, ___, parent_Lchild_pointer, parent_Rchild_pointer, node_pointer, ...]
    drop drop swap drop          # Drop unnecessary fields   [parent_Lchild_pointer, node_pointer, ...]
    eq                           # Check if node is Lchild   [isLeftChild_bool]
  else
    drop push.0  
  end            
end
 
proc.DereferenceParent
  # Inputs:  [node_pointer, ...]
  # Outputs: [...]
  #
  # Comments: remove any reference of node from parent
  #
  dup exec.getParentPointer     # [parent_pointer, node_pointer, ...]
  dup exec.SaveChangeFlag
  push.0 swap                   # [parent_pointer, 0, node_pointer, ...]
  movup.2 exec.isLeftChild      # [isLeftChild_bool, parent_pointer, 0, ...]
  if.true 
    exec.setLeftChildToParent   # [...]
  else 
    exec.setRightChildToParent  # [...]
  end
end

proc.setToNIL
  # Inputs:  [node_pointer, ...]
  # Outputs: [...]
  #
  # Comments: Zeros out node fields
  #
  dup exec.DereferenceParent   # [node_pointer, ...]
  push.0.0.0.0.0.0.0.0         # [0, 0, 0, 0, 0, 0, 0, 0, node_pointer, ...]  
  movup.8                      # [node_pointer, 0, 0, 0, 0, 0, 0, 0, 0, ...]
  exec.UpdateNode              # [...] 
end

proc.ZeroOut
  # Inputs:  [node_pointer, ...]
  # Outputs: [...]
  #
  # Comments: Zeros out node fields without attempting to dereference parents
  #
  push.0.0.0.0.0.0.0.0         # [0, 0, 0, 0, 0, 0, 0, 0, node_pointer, ...]  
  movup.8                      # [node_pointer, 0, 0, 0, 0, 0, 0, 0, 0, ...]
  exec.UpdateNode              # [...] 
end

proc.getSiblingPointer
  # Inputs:  [node_pointer, ...]
  # Outputs: [sibling_pointer, ...]
  #
  # Comments: Gets sibling pointer
  #
  dup exec.isLeftChild                                 # Is Node left child?               [isNodeLeftChild_bool, node_pointer, ...]
  swap exec.getParentPointer                           # Parent pointer                    [parent_pointer, isNodeLeftChild_bool, ...] 
  exec.getCoordinates exec.getChildrenFromCoordinates  # Parent's children pointers        [parent_Lchild_pointer, parent_Rchild_pointer, isNodeLeftChild_bool, ...]
  movup.2                                              # isNodeLeftChild_bool top of stack [isNodeLeftChild_bool, parent_Lchild_pointer, parent_Rchild_pointer, ...]
  if.true
    drop                                               # Sibling is parent's right child   [parent_Rchild_pointer, ...]
  else
    swap drop                                          # Sibling is parent's left child    [parent_Lchild_pointer, ...]
  end
end

proc.getNephewsPointers
  # Inputs:  [node_pointer, ...]
  # Outputs: [close_nephew_pointer, distant_nephew_pointer, ...]
  #
  # Comments: Gets nephew pointers, close nephew first
  #
  dup exec.isLeftChild not                             # Is Node Right child?              [isNodeRightChild_bool, node_pointer, ...]
  swap exec.getSiblingPointer                          # Sibling pointer                   [sibling_pointer, isNodeRightChild_bool, ...] 
  exec.getCoordinates exec.getChildrenFromCoordinates  # Sibling's children pointers       [sibling_Lchild_pointer, sibling_Rchild_pointer, isNodeRightChild_bool, ...]
  dup.2                                                # isNodeLeftChild_bool top of stack [isNodeRightChild_bool, sibling_Lchild_pointer, sibling_Rchild_pointer, ...]
  if.true
    swap                                               # Sibling's Right child is close nephew [sibling_Rchild_pointer, sibling_Lchild_pointer, ...]
  end
  #### Else # Sibling's Left child is close nephew [sibling_Lchild_pointer, sibling_Rchild_pointer, ...]
end

proc.hasLeftChild
  # Inputs: [node_pointer,....]
  # Ouputs: [hasLeftChild_bool, Lchild_pointer...]
  #
  # Comments: Verifies if node has left child and returns pointer to it
  #  
  exec.getCoordinates     # Get coordinates 
  drop drop swap drop     # Isolate Left child pointer
  dup push.0 eq not       # Duplicate and chech that it isn't NIL
end

proc.isPointerInCoordinate
  # Inputs: [node_pointer, coordinate_word, ...]
  # Ouputs: [isPointerInCoordinate_bool, Position_in_coordinate_word...]
  #
  # Comments: Checks if pointer exists in coordinate word and returns bool alongside position in word
  #
  swap drop               # Get rid of color               [node_pointer, parent_pointer, Lchild_pointer, Rchild_pointer, ...]
  dup.1 dup.1 neq         # Is node not the parent?        [isNodeParent_bool, node_pointer, parent_pointer, Lchild_pointer, Rchild_pointer, ...]
  if.true
    swap drop             # Drop parent pointer            [node_pointer, Lchild_pointer, Rchild_pointer, ...]
    dup.1 dup.1 neq       # Is node not the Left child?    [isNodeNotLchild_bool, node_pointer, Lchild_pointer, Rchild_pointer, ...]
    if.true 
      swap drop           # Drop Left child                [node_pointer, Rchild_pointer, ...]
      neq                 # Is node not the Right child    [isNodeNotRchild_bool, ...]
      if.true 
        push.0.0          # Node not in coordinate word    [0, 0, ...]
      else 
        push.3 push.1     # Node is Right child            [1, 3, ...]
      end
    else 
      drop drop drop
      push.2 push.1       # Node is Left child             [1, 2, ...]
    end 
  else 
    drop drop drop drop
    push.1 push.1         # Node is Parent                 [1, 1, ...]
  end
end

proc.SwapIntoPositionKofFour
  # Inputs: [K, value, elem1, ..., elem4 ...]
  # Ouputs: [elem1, ..., elemK=value, ..., elem4 ...]
  #
  # Comments: Swaps 'value' into list of elements {{elem1,...,elem4}} at position 'K'.
  #           For proper functioning, K<4
  #

  loc_store.0             # Save K                 [value, elem1, elem2, ..., elemN ...]
  push.1 loc_load.0 eq 
  if.true
    swap drop 
  else 
    push.2 loc_load.0 eq
    if.true 
      swap.2 drop      
    else
      push.3 loc_load.0 eq
      if.true 
        swap.3 drop     
      else 
        swap.4 drop      # K must equal 4
      end 
    end
  end 
end

proc.UpdateParentReference
  # Inputs: [nodeA_pointer, nodeB_pointer, ...]
  # Ouputs: [nodeA_pointer, nodeB_pointer, ...]
  #
  # Comments: Updates nodeB to new child of nodeA's parent.
  #
  dup exec.getParentPointer       # Get nodeA's parent                   [nodeA_parent_pointer, nodeA_pointer, nodeB_pointer, ...]
  dup exec.SaveChangeFlag         # Switch parent change flag
  dup.2 swap                      # Prepare for new reference            [nodeA_parent_pointer, nodeB_pointer, nodeA_pointer, nodeB_pointer, ...]
  dup.2 exec.isLeftChild          # Is A a left child?                   [isLeftChild_bool, nodeA_parent_pointer, nodeB_pointer, nodeA_pointer, nodeB_pointer, ...]
  if.true 
    exec.setLeftChildToParent     # Update child in parent               [nodeA_pointer, nodeB_pointer, ...]
  else 
    exec.setRightChildToParent    # Update child in parent               [nodeA_pointer, nodeB_pointer, ...]
  end
end

proc.UpdateChildReferences
  # Inputs: [nodeA_pointer, nodeB_pointer, ...]
  # Ouputs: [nodeA_pointer, nodeB_pointer, ...]
  #
  # Comments: Updates nodeB as new parent of nodeA's children.
  #
  dup exec.getCoordinates         # Get nodeA coordinates                [nodeA_coordiante_word, nodeA_pointer, nodeB_pointer, ...] 
  exec.getChildrenFromCoordinates # Isolate children                     [nodeA_Lchild, nodeA_Rchild, nodeA_pointer, nodeB_pointer, ...]
  dup exec.SaveChangeFlag
  dup.1 exec.SaveChangeFlag       
  dup.3                           # Duplicate nodeB pointer on stack     [nodeB_pointer, nodeA_Lchild, nodeA_Rchild, nodeA_pointer, nodeB_pointer, ...]
  swap dup.1                      # ReDuplicate the same                 [nodeB_pointer, nodeA_Lchild, nodeB_pointer, nodeA_Rchild, nodeA_pointer, nodeB_pointer, ...]
  exec.UpdateParentInChild        # Reassign parent of nodeA_Lchild      [nodeB_pointer, nodeA_Rchild, nodeA_pointer, nodeB_pointer, ...]
  exec.UpdateParentInChild        # Reassign parent of nodeA_Rchild      [nodeA_pointer, nodeB_pointer, ...]
end

proc.SwapFamilyReferences
  # Inputs: [nodeA_pointer, nodeB_pointer, ...]
  # Ouputs: [...]
  #
  # Comments: Updates nodeB to new child of nodeA's parent, and parent of nodeA' children. AND viceversa
  #
  exec.UpdateParentReference      # [nodeA_pointer, nodeB_pointer, ...]
  exec.UpdateChildReferences      # [nodeA_pointer, nodeB_pointer, ...]
  swap                            # [nodeB_pointer, nodeA_pointer, ...]
  exec.UpdateParentReference      # [nodeB_pointer, nodeA_pointer, ...]
  exec.UpdateChildReferences      # [nodeB_pointer, nodeA_pointer, ...]
  swap                            # [nodeA_pointer, nodeB_pointer, ...]
end

proc.SwapNodes
  # Inputs: [nodeA_pointer, nodeB_pointer, ...]
  # Ouputs: [...]
  #
  # Comments: Swaps coordinates of two nodes.
  #

  exec.SwapFamilyReferences

  dup exec.SaveChangeFlag         # Save nodeA change flag
  dup.1 exec.SaveChangeFlag       # Save nodeB change flag

  dup exec.getCoordinates         # Get node A coordinates               [nodeA_coordinate_word, nodeA_pointer, nodeB_pointer, ...]
  dup.5                           # Duplicate nodeB pointer on stack     [nodeB_pointer, nodeA_coordinate_word, nodeA_pointer, nodeB_pointer, ...]
  exec.isPointerInCoordinate      # Is nodeB is in direct family of A?   [isDirectFamilyMember_bool, BinA_position, nodeA_pointer, nodeB_pointer, ...]
  if.true
    push.1 add                    # Add one to BinA for consistency      [BinA_position, nodeA_pointer, nodeB_pointer, ...]
    dup.2 exec.getCoordinates     # Get node B coordinates               [nodeB_coordinate_word, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    dup.5                         # Duplicate node A pointer             [nodeA_pointer, nodeB_coordinate_word, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    exec.isPointerInCoordinate    # Is nodeA is in direct family of B?   [isDirectFamilyMember_bool, AinB_position, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    drop push.1 add               # Drop bool and fix AinB               [AinB_position, BinA_position, nodeA_pointer, nodeB_pointer, ...]

    dup.3 dup exec.getCoordinates # Get node B coordinates               [nodeB_coordinate_word, nodeB_pointer, AinB_position, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    movup.4                       # Move nodeB_pointer top of stack      [nodeB_pointer, nodeB_coordinate_word, AinB_position, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    movup.5                       # Move AinB to top of stack            [AinB_position, nodeB_pointer, nodeB_coordinate_word, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    exec.SwapIntoPositionKofFour  # Get new nodeA coordinate word        [new_nodeA_coordinate_word, BinA_position, nodeA_pointer, nodeB_pointer, ...]
    movup.5                       # Bring nodeA pointer top of stack     [nodeA_pointer, new_nodeA_coordinate_word, BinA_position, nodeB_pointer, ...]
    dup exec.getCoordinates       # Get nodeA coordiante word            [nodeA_coordinate_word, nodeA_pointer, new_nodeA_coordinate_word, BinA_position, nodeB_pointer, ...]
    movup.9                       # Bring BinA top of stack              [BinA_position, nodeA_coordinate_word, nodeA_pointer, new_nodeA_coordinate_word, nodeB_pointer, ...]
    dup.5 swap                    # Duplicate nodeA pointer              [BinA_position, nodeA_pointer, nodeA_coordinate_word, nodeA_pointer, new_nodeA_coordinate_word, nodeB_pointer, ...]
    exec.SwapIntoPositionKofFour  # Get new nodeB coordinate word        [new_nodeB_coordinate_word, nodeA_pointer, new_nodeA_coordinate_word, nodeB_pointer, ...]
    movup.9                       # Bring nodeB pointer top of stack     [nodeB_pointer, new_nodeB_coordinate_word, nodeA_pointer, new_nodeA_coordinate_word, ...]
    exec.SaveCoordinate           # Save new Node B coordinates          [nodeA_pointer, new_nodeA_coordinate_word, ...]
    exec.SaveCoordinate           # Save new node A coordinates          [...]
  else
    drop                          # Drop unnecessary position            [nodeA_pointer, nodeB_pointer, ...]
    dup exec.getCoordinates       # Get node A coordinates               [nodeA_coordinate_word, nodeA_pointer, nodeB_pointer, ...]
    movup.5                       # Bring Node B pointer top of stack    [nodeB_pointer, nodeA_coordinate_word, nodeA_pointer, ...]
    dup exec.getCoordinates       # Get node B coordinates               [nodeB_coordinate_word, nodeB_pointer, nodeA_coordinate_word, nodeA_pointer, ...]
    movup.9                       # Bring node A pointer top of stack    [nodeA_pointer, nodeB_coordinate_word, nodeB_pointer, nodeA_coordinate_word, ...]
    exec.SaveCoordinate           # Save new Node A coordinates          [nodeB_pointer, nodeA_coordinate_word, ...]
    exec.SaveCoordinate           # Save new node B coordinates          [...]
  end
end

proc.getRightMinimumSubtreeElement
  # Inputs: [node_pointer,....]
  # Ouputs: [Minimum_Right_subtree_pointer, ...]
  #
  # Comments: Starting from the node N, function traverses left subtree 
  #           to find minimum subtree node Y.
  #
  dup exec.getCoordinates         # Get node coordinates                   [node_coordinate_word, node_pointer, ...]
  drop drop drop                  # Isolate Right child                    [Rchild_pointer, node_pointer, ...]
  swap drop push.1                # Enter while loop                       [1, Rchild_pointer, ...]

  while.true 
    dup exec.hasLeftChild         # Is there a left child?                 [hasLeftChild_bool, Lchild_pointer, in-order_successor_pointer, ...]
    if.true 
      swap drop                   # Left child is new in-order successor   [Lchild_pointer, ...]
      push.1                      # Continue loop                          [1, Lchild_pointer, ...]
    else 
      drop                        # Current in-order successor is final    [in-order_successor_pointer, ...]
      push.0                      # Exit while loop                        [0, in-order_successor_pointer, ...]
    end
  end
end

proc.LeftRotate
  # Inputs: [pivot_node_pointer,....]
  # Ouputs: [...]
  #
  # For spec readability, recall that the structure of each node's coordinates is:
  #
  # {{color, parent_pointer, Left_child_pointer, Left_child_pointer}}
  #
  # In what follows, coordinate values which remain unchanged are shown as an underscore (____)
  # 
  # <Coordinates Before>
  # Node's Parent (NP):                      {{____,         ____,     N_pointer,        ____ }} or {{____, ____, ____ ,  N_pointer}}
  # Node coordinates (N):                    {{____,   NP_pointer,          ____, NRC_pointer }}
  # Node's Right Child (NRC):                {{____,    N_pointer, NRCLC_pointer,        ____ }}
  # Node's Right Child's Left Child (NRCLC): {{____,  NRC_pointer,          ____,        ____ }}
  #
  # <Coordinates After>
  # Node's Parent (NP):                      {{____,         ____,   NRC_pointer,          ____ }}  or {{____, ____, ____ ,  NRC_pointer}}
  # Node coordinates (N):                    {{____,  NRC_pointer,          ____, NRCLC_pointer }}
  # Node's Right Child (NRC):                {{____,   NP_pointer,     N_pointer,          ____ }}
  # Node's Right Child's Left Child (NRCLC): {{____,    N_pointer,          ____,          ____ }}
  #
  # Visual for reference see here:
  # https://en.wikipedia.org/wiki/Red%E2%80%93black_tree#/media/File:Binary_Tree_Rotation_(animated).gif
  #

  #### UPDATE Node Parent (RP) ####
  dup exec.getCoordinates                # Load N's coordinates                   [____, NP_pointer,  ____, NRC_pointer, N_pointer, ...]
  drop swap drop                         # Isolate NP_pointer and NRC_pointer     [NP_pointer, NRC_pointer,   N_pointer, ...]
  dup.2 exec.isLeftChild                 # Is node left child?                    [isLeftChild_bool, NP_pointer, NRC_pointer,   N_pointer, ...]
  if.true 
    dup.1 dup.1 exec.setLeftChildToParent  # Update NRC as NP's new left child    [NP_pointer, NRC_pointer,   N_pointer, ...]
  else 
    dup.1 dup.1 exec.setRightChildToParent  # Update NRC as NP's new left child   [NP_pointer, NRC_pointer,   N_pointer, ...]
  end

  #### UPDATE Node's Right Child's Left Child (NRCLC) ####
  dup.1 exec.getCoordinates                    # Load NRC coordinates             [         ____,     N_pointer, NRCLC_pointer,       ____,    NP_pointer, NRC_pointer,   N_pointer, ...]
  exec.getChildrenFromCoordinates swap drop    # Isolate pointer to NRCLC         [NRCLC_pointer,    NP_pointer,   NRC_pointer,  N_pointer, ...]
  dup dup.4                                    # N and NRCLC on top of stack      [    N_pointer, NRCLC_pointer, NRCLC_pointer, NP_pointer, NRC_pointer, N_pointer, ...]
  exec.UpdateParentInChild                     # Set N as new parent of NRCLC     [NRCLC_pointer, NP_pointer, NRC_pointer, N_pointer, ...]

  #### UPDATE Node (N) ####
  dup.3 exec.getCoordinates              # Load N coordinates                     [         ____,  NP_pointer,        ____,   NRC_pointer, NRCLC_pointer,  NP_pointer, NRC_pointer, N_pointer, ...]
  movup.4                                # Move NRCLC_pointer top of stack        [NRCLC_pointer,        ____,  NP_pointer,          ____,   NRC_pointer,  NP_pointer, NRC_pointer, N_pointer, ...]
  swap.4 drop                            # Set NRCLC as right child of N          [         ____,  NP_pointer,        ____, NRCLC_pointer,    NP_pointer, NRC_pointer, N_pointer, ...]
  dup.5                                  # Duplicate NRC_pointer                  [  NRC_pointer,        ____,  NP_pointer,          ____, NRCLC_pointer,  NP_pointer, NRC_pointer, N_pointer, ...] 
  swap.2 drop                            # Set NRC as parent of N                 [         ____, NRC_pointer,        ____, NRCLC_pointer,    NP_pointer, NRC_pointer, N_pointer, ...]
  dup.6 exec.UpdateCoordinate            # Save new coordinates of N              [   NP_pointer, NRC_pointer,   N_pointer, ...]   

  #### UPDATE Node's Right Child (NRC) ####
  dup.1 exec.getCoordinates              # Load NRC coordinates                   [       ____,  N_pointer, NRCLC_pointer,      ____,  NP_pointer, NRC_pointer, N_pointer, ...]
  movup.4 swap.2 drop                    # Set NP as new parent of NRC            [       ____, NP_pointer, NRCLC_pointer,      ____, NRC_pointer,   N_pointer, ...]
  movup.5 swap.3 drop                    # Set N as new left child of NRC         [       ____, NP_pointer,     N_pointer,      ____, NRC_pointer, ...]
  movup.4 exec.UpdateCoordinate          # Save new coordinates of NRC            [ ...]
end

proc.RightRotate
  # Inputs: [pivot_node_pointer,....]
  # Ouputs: [...]
  #
  # For spec readability, recall that the structure of each node's coordinates is:
  #
  # {{color, parent_pointer, Left_child_pointer, Left_child_pointer}}
  #
  # In what follows, coordinate values which remain unchanged are shown as an underscore (____)
  # 
  # <Coordinates Before>
  # Root's Parent (NP):                      {{____,         ____,     N_pointer,         ____ }}  or {{____, ____, ____ ,  N_pointer}}
  # Root coordinates (N):                    {{____,   NP_pointer,   NLC_pointer,         ____ }}
  # Root's Left Child (NLC):                 {{____,    N_pointer,          ____, NLCRC_pointer}}
  # Root's Left Child's Right Child (NLCRC): {{____,  NLC_pointer,          ____,         ____ }}
  #
  # <Coordinates After>
  # Root's Parent (NP):                      {{____,         ____,   NLC_pointer,         ____ }}  or {{____, ____, ____ ,  NLC_pointer}}
  # Root coordinates (N):                    {{____,  NLC_pointer, NLCRC_pointer,         ____ }}
  # Root's Left Child (NLC):                 {{____,   NP_pointer,          ____,     N_pointer}}
  # Root's Left Child's Right Child (NLCRC): {{____,    N_pointer,          ____,         ____ }}
  #
  # Visual for reference see here:
  # https://en.wikipedia.org/wiki/Red%E2%80%93black_tree#/media/File:Binary_Tree_Rotation_(animated).gif
  #

  #### UPDATE Root Parent (NP) ####
  dup exec.getCoordinates                # Load N's coordinates                   [____,   NP_pointer,   NLC_pointer,  ____, N_pointer, ...]
  drop movup.2 drop                      # Isolate NP_pointer and NLC_pointer     [NP_pointer,   NLC_pointer,   N_pointer, ...]
  dup.2 exec.isLeftChild                 # Is node left child?                    [isLeftChild_bool, NP_pointer, NLC_pointer,   N_pointer, ...]
  if.true 
    dup.1 dup.1 exec.setLeftChildToParent  # Update NRC as NP's new left child    [NP_pointer, NLC_pointer,   N_pointer, ...]
  else 
    dup.1 dup.1 exec.setRightChildToParent  # Update NRC as NP's new left child   [NP_pointer, NLC_pointer,   N_pointer, ...]
  end

  #### UPDATE Root's Left Child's Right Child (NLCRC) ####
  dup.1 exec.getCoordinates              # Load NLC coordinates                   [         ____,     N_pointer,          ____, NLCRC_pointer,  NP_pointer, NRC_pointer,   N_pointer, ...]
  exec.getChildrenFromCoordinates drop   # Isolate pointer to NLCRC               [NLCRC_pointer,    NP_pointer,   NLC_pointer,     N_pointer, ...]
  dup dup.4                              # N and NLCRC pointer on stack           [    N_pointer, NLCRC_pointer, NLCRC_pointer,    NP_pointer, NLC_pointer,   N_pointer, ...]
  exec.UpdateParentInChild               # Set N as new parent of NLCRC           [NLCRC_pointer,    NP_pointer,   NLC_pointer,     N_pointer, ...]

  #### UPDATE Root (N) ####
  dup.3 exec.getCoordinates              # Load N coordinates                     [         ____,  NP_pointer,   NLC_pointer,          ____, NLCRC_pointer,  NP_pointer, NLC_pointer, N_pointer, ...]
  movup.4                                # Move NLCRC_pointer top of stack        [NLCRC_pointer,        ____,    NP_pointer,   NLC_pointer,          ____,  NP_pointer, NLC_pointer, N_pointer, ...]
  swap.3 drop                            # Set NLCRC as left child of N           [         ____,  NP_pointer, NLCRC_pointer,          ____,    NP_pointer, NLC_pointer,   N_pointer, ...]
  dup.5                                  # Duplicate NLC_pointer                  [  NLC_pointer,        ____,    NP_pointer, NLCRC_pointer,          ____,  NP_pointer, NLC_pointer, N_pointer, ...] 
  swap.2 drop                            # Set NLC as parent of N                 [         ____, NLC_pointer, NLCRC_pointer,          ____,    NP_pointer, NLC_pointer,   N_pointer, ...]
  dup.6 exec.UpdateCoordinate            # Save new coordinates of N              [NP_pointer, NLC_pointer, N_pointer, ...]   

  # #### UPDATE Root's Left Child (NLC) ####
  dup.1 exec.getCoordinates              # Load NLC coordinates                   [       ____,  N_pointer,       ____, NLCRC_pointer, NP_pointer, NLC_pointer, N_pointer, ...]
  movup.4 swap.2 drop                    # Set NP as new parent of NLC            [       ____, NP_pointer,       ____, NLCRC_pointer, NLC_pointer,   N_pointer, ...]
  movup.5 swap.4 drop                    # Set N as new right child of NLC        [       ____, NP_pointer,       ____,     N_pointer, NLC_pointer, ...]
  movup.4 exec.UpdateCoordinate          # Save new coordinates of NRC            [ ...]
end

proc.BalanceTree
  # Inputs: [node_pointer,....]
  # Ouputs: [...]
  #
  # Comments: The main goal of this procedure is to maintain the 
  #           Red-Black tree properties after an insertion. 
  #           It assumes that node_pointer points to a current 
  #           node that has just been inserted.
  #           Operation rebalances tree to satisfy rbBST conditions:
  # 
  #                                1) Every node is either red or black.
  #                                2) All NIL nodes (figure 1) are considered black.
  #                                3) A red node does not have a red child.
  #                                4) Every path from a given node to any of its descendant NIL 
  #                                   nodes goes through the same number of black nodes.
  #                                5) (Conclusion) If a node N has exactly one child, it must be 
  #                                   a red child, because if it were black, its NIL descendants 
  #                                   would sit at a different black depth than N's NIL child, violating 
  #                                   requirement 4. 
  #

  dup exec.isRoot                                    # node is root       [isRoot_bool, node_pointer, ...]
  if.true
    push.0                                           # Avoid while loop   [0, node_pointer, ...]
  else
    dup exec.getParentPointer exec.getColor        # get parent color   [parent_color, node_pointer, ...]
  end
  
  #
  # Case 1: Node is Root or parent is black. If node is root, it just needs to be painted black. 
  #         This takes place at the bottom of function 
  #
  while.true                                       # node isn't root and parent is red  [node_pointer, ...]
    #
    # Conditional summary: Node (not root, red), Parent (red)
    #
    dup exec.getParentPointer exec.isLeftChild     # Is parent a left child?   [isParentLeftChild_bool, node_pointer, ...]

    if.true
    #
    # Conditional summary: Node (not root, red), Parent (red, left child)
    #
      dup exec.isUncleNIL not                      # Is Uncle not NIL            [isUncleNotNIL_bool, node_pointer, ...] 
      dup.1 exec.getUnclePointer exec.getColor     # Load Uncle color            [uncle_color, isUncleNotNIL_bool, node_pointer, ...]
      and                                          # Check that uncle exists and is red also  

      if.true                                      # IF uncle != NIL and uncle_color = red
        #
        # Conditional summary: Node (not root, red), Parent (red, left child), Uncle (not NIL, red, right child)
        #
        #
        # Case 2: Set parent (P) and uncle (U) colors to black, and grandparent (G) color to red. 
        # If both the parent and the uncle are red, then both of them can be repainted 
        # black and the grandparent becomes red for maintaining requirement 4. G becomes the new
        # node pointer at the end of the case. 
        #
        push.0 dup.1 exec.getUnclePointer        # Get uncle pointer & color    [uncle_pointer, 0, node_pointer, ...]
        exec.setColor                            # Set Uncle color to black     [node_pointer, ...]
        exec.getParentPointer                    # Get parent pointers          [parent_pointer, ...]
        push.0 dup.1                             # Get parent pointer & color   [parent_pointer, 0, parent_pointer, ...]
        exec.setColor                            # Set Parent color to black    [parent_pointer, ...]
        exec.getParentPointer push.1 dup.1       # Get pointer to grandparent   [grandparent_pointer, 1, grandparent_pointer, ...]
        exec.setColor                            # Set grandparent to red       [grandparent_pointer, ...]

      else
        #
        # Conditional summary: Node (not root, red), Parent (red, left child), Uncle (black)
        #
        # The parent P is red but the uncle U is black. The ultimate goal is to rotate the parent 
        # node P to the grandparent position, but this will not work if N is an inner grandchild 
        # of G (i.e., if N is the left child of the right child of G or the right child of the left 
        # child of G).  
        #
        dup exec.isLeftChild not                    # Check if node is Right child [isRightChild_bool, node_pointer, ...]  
        if.true
          #
          # Conditional summary: Node (not root, red, right child), Parent (red, left child), Uncle (black)
          #
          # Case 2: N is a left child while P is a right child. By rotating right using P
          # as pivot, we create a red-red parent child combination that is aligned. The case exits
          # by setting P as the new reference pointer. 
          #
          exec.getParentPointer                 # Set rotation pivot to parent           [parent_pointer, ...]
          dup exec.LeftRotate                   # Rotate tree left around parent_pointer [parent_pointer, ...]
        end

        #
        # Conditional summary: Node (not root, red, right child), Parent (red, right child), Uncle (black)
        #
        # Case 3: N and P are both left children. P is set to black, G to red, and tree is rotated
        # right around G.  The case exits with G as the new reference pointer.
        #
        exec.getParentPointer push.0 dup.1       # Prepare setting parent black          [parent_pointer, 0, parent_pointer, ...]
        exec.setColor                            # Set parent color                      [parent_pointer, ...]
        exec.getParentPointer push.1 dup.1       # Prepare setting grandparent red       [grandparent_pointer, 1, grandparent_pointer, ...]
        exec.setColor                            # Set grandparent color                 [grandparent_pointer, ...]
        dup exec.RightRotate                      # Rotate tree right around grandparent [grandparent_pointer, ...]
      end

    else  # Mirror image of the above cases
      #
      # Conditional summary: Node (not root, red), Parent (red, right child)
      #
      # Case 3: N and P are both left children. P is set to black, G to red, and tree is rotated
      # left around G.  The case exits with G as the new reference pointer.
      #
      dup exec.isUncleNIL not                      # Is Uncle not NIL            [isUncleNotNIL_bool, node_pointer, ...] 
      dup.1 exec.getUnclePointer exec.getColor     # Load Uncle color            [uncle_color, isUncleNotNIL_bool, node_pointer, ...]
      and                                          # Check that uncle exists and is red also                          
      if.true                                      # IF uncle != NIL and uncle_color = red
        #
        # Conditional summary: Node (not root, red), Parent (red, right child), Uncle (not NIL, red, left child)
        #
        #
        # Case 2 as above
        #
        push.0 dup.1 exec.getUnclePointer        # Get uncle pointer & color    [uncle_pointer, 0, node_pointer, ...]
        exec.setColor                            # Set Uncle color to black     [node_pointer, ...]
        exec.getParentPointer                    # Get parent pointers          [parent_pointer, ...]
        push.0 dup.1                             # Get parent pointer & color   [parent_pointer, 0, parent_pointer, ...]
        exec.setColor                            # Set Parent color to black    [parent_pointer, ...]
        exec.getParentPointer push.1 dup.1       # Get pointer to grandparent   [grandparent_pointer, 1, grandparent_pointer, ...]
        exec.setColor                            # Set grandparent to red       [grandparent_pointer, ...] 
      else
        #
        # Conditional summary: Node (not root, red), Parent (red, right child), Uncle (black, left child)
        #
        #
        dup exec.isLeftChild                     # Check if node is Left child [isLeftChild_bool, node_pointer, ...]
        if.true
          #
          # Conditional summary: Node (not root, red, left child), Parent (red, right child), Uncle (black, left child)
          #
          exec.getParentPointer                  # Set reference pointer to parent [parent_pointer, ...]
          dup exec.RightRotate                    # Rotate tree Left around parent  [parent_pointer, ...]
        end
        #
        # Conditional summary: Node (not root, red, left child), Parent (red, right child), Uncle (black, left child)
        #
        exec.getParentPointer push.0 dup.1       # P->black                     [parent_pointer, 0, parent_pointer, ...]
        exec.setColor                            # Set Color                    [parent_pointer, ...]
        exec.getParentPointer push.1 dup.1       # G->red                       [grandparent_pointer, 1, grandparent_pointer, ...]
        exec.setColor                            # Set Color                    [grandparent_pointer, ...]
        dup exec.LeftRotate                     # Rotate tree right around G    [grandparent_pointer, ...]
       end
    end

    dup exec.isRoot not                              # node isn't root    [ isRoot_bool, node_pointer, ...]
    if.true 
      dup exec.getParentPointer exec.getColor        # get parent color   [parent_color, isRoot_bool, node_pointer, ...]
    else                                              # Continue while loop if Node isn't root and parent is red
      push.0
    end
  end                                                # While loop exits as  [root_node_pointer, ...]

  ### CONTINUATION OF CASE 1 FROM THE TOP ###
  dup exec.isRoot
  if.true
    push.0 swap
    exec.setColor                                   # Root node should always be black  [...] 
  else
    drop                                             
  end                                              
end

proc.DeleteNode_CaseD6
  # Inputs:  [node_pointer, ...]
  # Outputs: [node_pointer, ...]
  #
  # Comments: The sibling S is black, S’s distant child D is red. 
  #           After a dir-rotation at P the sibling S becomes the 
  #           parent of P and S’s distant child D. The colors of P 
  #           and S are exchanged, and D is made black. The whole 
  #           subtree still has the same color at its root S, namely 
  #           either red or black ( in the diagram), which refers 
  #           to the same color both before and after the transformation. 
  #           This way requirement 3 is preserved. The paths in the 
  #           subtree not passing through N pass through the same number 
  #           of black nodes as before, but N now has one additional 
  #           black ancestor: either P has become black, or it was black 
  #           and S was added as a black grandparent. Thus, the paths 
  #           passing through N pass through one additional black node, 
  #           so that requirement 4 is restored and the total tree is in RB-shape. 
  #
  dup exec.getParentPointer              # Get parent pointer                      [parent_pointer, node_pointer, ...]
  dup.1 exec.getSiblingPointer           # Get Sibling Pointer                     [sibling_pointer, parent_pointer, node_pointer, ...]
  dup exec.getCoordinates                # Get Sibling coordinates                 [sibling_coordinate_word, sibling_pointer, parent_pointer, node_pointer, ...]
  exec.getChildrenFromCoordinates        # Get nephews                             [Lnephew_pointer, Rnephew_pointer, sibling_pointer, parent_pointer, node_pointer, ...]
  dup.4 exec.isLeftChild                 # Is node left child                      [isLeftChild_bool, Lnephew_pointer, Rnephew_pointer, sibling_pointer, parent_pointer, node_pointer, ...] 
  if.true
    drop loc_store.2                     # Rnephew is distant. Save.               [sibling_pointer, parent_pointer, node_pointer, ...]
    loc_store.1 dup loc_store.0          # Save sibling and parent                 [parent_pointer, node_pointer, ...]
    exec.LeftRotate                      # Perform Left rotation around parent     [node_pointer, ...]
  else
    swap drop loc_store.2                # Lnephew is distant. Save.               [sibling_pointer, parent_pointer, node_pointer, ...]
    loc_store.1 dup loc_store.0          # Save sibling and parent                 [parent_pointer, node_pointer, ...] 
    exec.RightRotate                     # Perform Right rotation around parent    [node_pointer, ...]
  end 

  loc_load.0 dup exec.getColor               # Get parent color                      [parent_color, parent_pointer, node_pointer, ...]
  loc_load.1                                 # Get two Sibling pointers              [sibling_pointer, sibling_pointer, parent_color, parent_pointer, node_pointer, ...]
  exec.setColor                              # Color sibling like parent             [parent_pointer, node_pointer, ...]
  loc_load.2                                 # Load two distant nephew pointer       [distant_nephew_pointer, distant_nephew_pointer, parent_pointer, node_pointer, ...]
  push.0 swap exec.setColor                  # Color distant nephhew BLACK           [parent_pointer, node_pointer, ...]
  push.0 swap exec.setColor                  # Color parent BLACK                    [node_pointer,...]
end

proc.DeleteNode_CaseD5
  # Inputs:  [node_pointer, ...]
  # Outputs: [node_pointer, ...]
  #
  # Comments: The sibling S is black, S’s close child C is red, and S’s distant child D is black. 
  #           After a (1-dir)-rotation at S the nephew C becomes S’s parent and N’s new sibling. 
  #           The colors of S and C are exchanged. All paths still have the same number of black nodes, 
  #           but now N has a black sibling whose distant child is red, so the constellation is fit for 
  #           case D6. Neither N nor its parent P are affected by this transformation, and P may be red 
  #           or black. 
  dup exec.getSiblingPointer            # Get sibling pointer                     [sibling_pointer, node_pointer, ...]
  dup exec.getCoordinates               # Get sibling coordiante word             [sibling_coordinate_word, sibling_pointer, node_pointer, ...]
  exec.getChildrenFromCoordinates       # Get nephew pointers                     [Lnephew_pointer, Rnephew_pointer, sibling_pointer, node_pointer, ...]
  dup.2 exec.isLeftChild                # Is node Left child?                     [isLeftChild_bool, Lnephew_pointer, Rnephew_pointer, sibling_pointer, node_pointer, ...]
  if.true 
    #
    # Comment: Left nephew is close Nephew
    #
    swap drop                           # Drop distant nephew                     [closeNephew_pointer, sibling_pointer, node_pointer, ...]
    dup.1 exec.RightRotate              # Right Rotate around sibling             [closeNephew_pointer, sibling_pointer, node_pointer, ...]
  else
    #
    # Comment: Right nephew is close Nephew
    #
    drop                                # Swap Left and Right nephews             [closeNephew_pointer, sibling_pointer, node_pointer, ...]
    dup.2 exec.LeftRotate               # Left Rotate around sibling              [closeNephew_pointer, sibling_pointer, node_pointer, ...]
  end
  push.0 swap                           # Save close nephew change flag           [closeNephew_pointer, 0, sibling_pointer, node_pointer, ...]
  exec.setColor                         # Color close nephew BLACK                [sibling_pointer, node_pointer, ...]
  push.1 swap                           # Save sibling change flag                [sibling_pointer, 1, node_pointer, ...]
  exec.setColor                         # Color sibling RED                       [node_pointer, ...]
  exec.DeleteNode_CaseD6                # Go to delete case D6                    [node_pointer, ...]
end

proc.DeleteNode_CaseD4
  # Inputs:  [node_pointer, ...]
  # Outputs: [node_pointer, ...]
  #
  # Comments: The sibling S and S’s children are black, but P is red. Exchanging the colors of 
  #           S and P does not affect the number of black nodes on paths going through S, but 
  #           it does add one to the number of black nodes on paths going through N, making up 
  #           for the deleted black node on those paths. 

  dup exec.getSiblingPointer           # Get Sibling pointer                      [sibling_pointer, node_pointer, ...]
  push.1 swap                          # Save sibling change flag
  exec.setColor                        # Color sibling RED                        [node_pointer, ...]
  dup exec.getParentPointer            # Get Parent pointer                       [parent_pointer, node_pointer, ...]
  push.0 swap                          # Save parent chagne flag                  [parent_pointer, 0, node_pointer, ...]  
  exec.setColor                        # Color parent BLACK                       [node_pointer, ...]
end
 
proc.DeleteNode_CaseD3
  # Inputs:  [node_pointer, ...]
  # Outputs: [node_pointer, ...]
  #
  # Comments: The sibling S is red, so P and the nephews C and D have to be black. 
  #           A dir-rotation at P turns S into N’s grandparent. Then after reversing 
  #           the colors of P and S, the path through N is still short one black node. 
  #           But N now has a red parent P and after the reassignment a black sibling S, 
  #           so the transformations in cases D4, D5, or D6 are able to restore the RB-shape. 
  #
  dup exec.getSiblingPointer            # Get sibling pointer                     [sibling_pointer, node_pointer, ...]
  dup.1                                 # Get copy of node pointer on stack       [node_pointer, sibling_pointer, node_pointer, ...]
  exec.getParentPointer                 # Get parent pointers                     [parent_pointer, sibling_pointer, node_pointer, ...]
  dup.2 exec.isLeftChild                # Is node left child                      [isLeftChild_bool, parent_pointer, sibling_pointer, node_pointer, ...]
  if.true            
    dup exec.LeftRotate                 # Left rotate using parent as pivot       [parent_pointer, sibling_pointer, node_pointer, ...]
  else 
    dup exec.RightRotate                # Right rotate using parent as pivot      [parent_pointer, sibling_pointer, node_pointer, ...]
  end   
  push.1 swap                           # Prepare RED color setting for parent    [parent_pointer, 1, sibling_pointer, node_pointer, ...]
  exec.setColor                         # Set parent to RED                       [sibling_pointer, node_pointer, ...]
  movup.2 push.0 swap                   # Moveup sibling pointer and prep BLACK   [sibling_pointer, 0, node_pointer, ...]
  exec.setColor                         # Set sibling to BLACK                    [node_pointer, ...]
  exec.getSiblingPointer                # Get new sibling pointer                 [newSibling_pointer, node_pointer, ...]
  exec.getCoordinates 
  exec.getChildrenFromCoordinates swap  # Get nephew pointers                     [distantNephew_pointer, closeNephew_pointer, node_pointer, ...]

  dup exec.isNIL not                    # Distant nephew not NIL?                 [isDistantNephewNotNIL_bool, distantNephew_pointer, closeNephew_pointer, node_pointer, ...]
  swap exec.getColor push.1 eq          # Distant nephew RED?                     [isDistantNephewRED_bool, isDistantNephewNotNIL_bool, closeNephew_pointer, node_pointer, ...]
  and                                   # Distant nephew RED and not NIL          [isDistantNephewREDAndNotNIL_bool, closeNephew_pointer, node_pointer, ...]
  if.true   
    drop exec.DeleteNode_CaseD6         # Go to delete case D6                    [node_pointer, ...]
  else
    dup exec.isNIL not swap             # Is close nephew not NIL?                [closeNephew_pointer, isCloseNephewNotNIL_bool, node_pointer, ...] 
    exec.getColor push.1 eq             # Is close nephew RED?                    [isCLoseNephewRED_bool, isCloseNephewNotNIL_bool, node_pointer, ...]
    and                                 # Is close nephew RED and not NIL?        [isCloseNephewREDAndNotNIL_bool, node_pointer, ...]
    if.true 
      exec.DeleteNode_CaseD5            # Go to delete case D5                    [node_pointer, ...]
    else 
      exec.DeleteNode_CaseD4            # Go to delete case D4                    [node_pointer, ...]
    end
  end                    
end

proc.DeleteNode
  # Inputs:  [node_pointer, ...]
  # Outputs: [node_pointer, recursion_bool...]
  #
  # Comments: Subroutine for deleting nodes. This gets called by 'proc.DestroyNode'
  #

  ###################################################################
  #                                                                 #
  #                         SIMPLE CONDITIONS                       #
  #                                                                 #
  ###################################################################

  ### NODE IS ROOT AND HAS NO CHILDREN ### 
  #
  # Comment: Zero out node and dereference from initTree node
  #
  dup exec.isRoot              # Node is Root                            [isNodeRoot_bool, node_pointer, ...]
  dup.1 exec.HasChildren not   # Node has no Children                    [isNodeWithChildren, isNodeRoot_bool, node_pointer, ...]
  and                          # Node is Root and No Children            [isNodeRootandNoChildren_bool, node_pointer, ...]
  if.true
    dup exec.setToNIL dup      # Zero out fields                         [node_pointer, node_pointer, ...]
    push.0 exec.getNodePointer # Get InitTree pointer                    [initTree_pointer, node_pointer, node_pointer, ...]
    exec.UpdateChildInParent   # Update child reference in initTree node [node_pointer, ...]
    push.0 swap                # Set recursion flag                      [node_pointer, 0, ...]
    push.0                     # Skip next cases                         [0, node_pointer, 0, ...]
  else
    push.1                     # Push 1 to go to next case               [1, node_pointer, ...]
  end

  ### NODE HAS ONLY ONE CHILD ###
  #
  # Comment: If node has only one child, due to rbBST properties, single child must be colored RED.
  #          Its parent must be BLACK. Current node needs to be zerod out, parent reference in child 
  #          and child reference in parent should both be updated. Child should be colored BLACK 
  #
  if.true
    ### CONDITION CHECK ###
    dup exec.getCoordinates              # Get node coordinates            [node_coordinate_word, node_pointer, ...]
    exec.getChildrenFromCoordinates      # Get children coordinates        [Lchild_pointer, Rchild_pointer, node_pointer, ...]
    dup.1 dup.1                          # Duplicate children pointers     [Lchild_pointer, Rchild_pointer, Lchild_pointer, Rchild_pointer, node_pointer, ...]
    push.0 eq                            # Is Left child NIL?              [isLeftChildNIL_bool, Rchild_pointer, Lchild_pointer, Rchild_pointer, node_pointer, ...]
    swap push.0 eq                       # Is Right child NIL?             [isRightChildNIL_bool, isLeftChildNIL_bool, node_pointer, ...]
    xor                                  # Has only one child              [isHasSingleChild_bool, Lchild_pointer, Rchild_pointer, node_pointer, ...]
    if.true
      #
      # Comment: Node has only one child
      #

      dup push.0 eq                      # Is left child NIL               [isLeftChildNIL_bool, Lchild_pointer, Rchild_pointer, node_pointer, ...]
      if.true
        drop                             # Isolate Right child             [singleChild_pointer, node_pointer, ...]
      else
        swap drop                        # Isolate Left child              [singleChild_pointer, node_pointer, ...]
      end

      dup push.0 swap exec.setColor      # Color Single child BLACK        [singleChild_pointer, node_pointer, ...]
      dup.1 exec.isLeftChild             # Is node left child?             [isLeftChild_bool, singleChild_pointer, node_pointer, ...]
      dup.2 dup exec.getParentPointer    # Get parent pointer              [parent_pointer, node_pointer, isLeftChild_bool, singleChild_pointer, node_pointer, ...] 
      swap exec.setToNIL                 # Zero out node.                  [parent_pointer, isLeftChild_bool, singleChild_pointer, node_pointer, ...]
      
      movup.2 swap                       # Send bool to backwards          [parent_pointer, singleChild_pointer, isLeftChild_bool,  node_pointer, ...]
      dup.1 dup.1                        # Duplicate parent/single child   [parent_pointer, singleChild_pointer, parent_pointer, singleChild_pointer, isLeftChild_bool,  node_pointer, ...]
      exec.UpdateParentInChild           # Update parent in child          [parent_pointer, singleChild_pointer, isLeftChild_bool, node_pointer, ...]
      movup.2                            # Move bool to top of stack       [isLeftChild_bool, parent_pointer, singleChild_pointer, node_pointer, ...]
      if.true
        exec.setLeftChildToParent        # Update child in parent          [node_pointer, ...]
      else
        exec.setRightChildToParent       # Update child in parent          [node_pointer, ...]
      end
      push.0 swap                        # Set recursion flag              [node_pointer, 0, ...]
      push.0                             # Skip next cases                 [0, node_pointer, 0, ...]
    else
      drop drop                          # Get rid of child pointers       [node_pointer, ...]
      push.1                             # Push 1 to go to next case       [1, node_pointer, ...] 
    end
  else
    push.0                               # Push 0 to skip next cases also  [0, node_pointer, 0, ...] 
  end


  if.true
    
    ###################################################################
    #                                                                 #
    #                         COMPLEX CONDITIONS                      #
    #                                                                 #
    ###################################################################

    dup exec.getColor                    # Get Node color                          [node_color, node_pointer, ...]
    if.true
      ### NODE IS RED ###
      #
      # Comment: If N is a red node, it cannot have exactly one non-NIL child, 
      #          because this would have to be black by requirement 3. Furthermore, 
      #          it cannot have exactly one black child according to conclusion 5. 
      #          As a consequence, the red node N is without any child and can simply 
      #          be removed, or has two black children and requires recursion into its
      #          subtree to swap with its in-order successor.   
      #
      dup exec.HasChildren
      if.true
        dup exec.getRightMinimumSubtreeElement   # Get minimum subtree elem          [minElem_pointer, node_pointer, ...]
        dup.1 exec.SwapNodes                     # Swap nodes                        [minElem_pointer, node_pointer, ...]
        push.1 swap                              # Set recursion flag                [node_pointer, 1, ...]
      else
        push.0 swap                        # Set recursion flag                      [node_pointer, 0, ...]
        dup exec.setToNIL                  # Zero out the node                       [node_pointer, ...]
      end

    else
      ### NODE IS BLACK ###
      #
      # Comment: By the rbBST rules, if N is a black node, it may ONLY have: 
      #                  A) a single red child, 
      #                  B) two children,
      #                  C) no children at all.  
      #

      dup exec.getCoordinates            # Load Coordinates                        [node_coordinate_word, node_pointer, ...]
      exec.getChildrenFromCoordinates    # Get Child pointers                      [Lchild_pointer, Rchild_pointer, node_pointer, ...]

      ### CHECK FOR SINGLE CHILD ###
      dup.1 dup.1                        # Copy child coordinates                  [Lchild_pointer, Rchild_pointer, Lchild_pointer, Rchild_pointer, node_pointer, ...]
      push.0 eq                          # No Left child?                          [noLchild_bool, Rchild_pointer, Lchild_pointer, Rchild_pointer, node_pointer, ...] 
      swap push.0 eq                     # No Right child?                         [noRchild_bool, noLchild_bool, Lchild_pointer, Rchild_pointer, node_pointer, ...]
      xor                                # Single child?                           [hasSingleChild_bool, Lchild_pointer, Rchild_pointer, node_pointer, ...]
      if.true
        #
        #        CASE A: N has a single red child. Replace N with child after painting the latter black. 
        #

        dup push.0 eq                    # Is Left child NIL                       [isLeftChildNIL_bool, Lchild_pointer, Rchild_pointer, node_pointer, ...]
        if.true
          drop                           # Right child is the one                  [Rchild_pointer, node_pointer, ...]
        else 
          swap drop                      # Left child is the one                   [Lchild_pointer, node_pointer, ...]
        end

        dup.1 exec.getParentPointer    # Get parent pointer                      [parent_pointer, singlechild_pointer, node_pointer, ...]
        dup.2 dup exec.isLeftChild     # Is node Left child?                     [isLeftChild_bool, node_pointer, parent_pointer, singlechild_pointer, node_pointer, ...] 
        swap exec.setToNIL             # Zero out node                           [isLeftChild_bool, parent_pointer, singlechild_pointer, node_pointer, ...]
        dup.2 push.0 swap              # Duplicate child and set black on stack  [singlechild_pointer, 0, isLeftChild_bool, parent_pointer, singlechild_pointer, node_pointer, ...] 
        exec.setColor                  # Set child color to black                [isLeftChild_bool, parent_pointer, singlechild_pointer, node_pointer, ...]
        dup.2 dup.2                    # Duplicate parent and child pointers     [parent_pointer, singlechild_pointer, isLeftChild_bool, parent_pointer, singlechild_pointer, node_pointer, ...]
        exec.UpdateParentInChild       # Parent of N becomes parent of child     [isLeftChild_bool, parent_pointer, singlechild_pointer, node_pointer, ...]
        if.true
          exec.setLeftChildToParent    # Single child becomes new Left child     [node_pointer, ...]
        else 
          exec.setRightChildToParent   # Single child becomes new Right child    [node_pointer, ...]
        end
        push.0 swap                    # Set recursion flag                      [node_pointer, 0, ...]

      else
        #
        # CONDITION SUMMARY: Black node with either two children or no children at all
        #
        dup.2 exec.HasChildren         # Does N have children?                   [hasChildren_bool, Lchild_pointer, Rchild_pointer, node_pointer, ...]
        if.true
          #
          #         CASE B: N has two children. If N has two non-NIL children, 
          #                 an additional navigation to the minimum element in its right subtree 
          #                 to N’s in-order successor (which we call Y) is required. 
          #                 Y does not have a left child and thus has at most one non-NIL child. 
          #                 If Y is to be removed in N’s place, the red–black tree data related 
          #                 with N and Y, i.e. the color of and the pointers to and from the two nodes, 
          #                 have to be exchanged. As a result, the modified red–black tree is the 
          #                 same as before, except that the order between N and Y is reversed.)
          #
          drop drop dup                            # Reorganize stack                [node_pointer, node_pointer, ...]
          exec.getRightMinimumSubtreeElement       # Get minimum subtree elem        [minElem_pointer, node_pointer, ...]
          dup.1 dup.1 exec.SwapNodes               # Swap nodes                      [minElem_pointer, node_pointer, ...]
          drop                                     # Isolate node pointer            [node_pointer, ...]
          push.1 swap                              # Set recursion flag              [node_pointer, 1, ...]

        else
          #
          #         CASE C: N has no children
          #

          drop drop dup                              # Duplicate node pointer          [node_pointer, node_pointer, ...]
          exec.getSiblingPointer dup                 # get Sibling pointer             [sibling_pointer, sibling_pointer, node_pointer, ...]
          exec.getColor                            # get Sibling color               [sibling_color, sibling_pointer, node_pointer, ...]
          if.true 
          #   #
          #   # CONDITIONAL SUMMARY: Sibling is a RED node. so P and the nephews C and D have to be black. 
          #   #                      An appropriate rotation at P turns S into N’s grandparent. Then after reversing 
          #   #                      the colors of P and S, the path through N is still short one black node.
          #   #
          drop exec.DeleteNode_CaseD3            # Go to delete case D3            [node_pointer, ...]
          else
            #
            # Sibling is BLACK. 
            # 
            exec.getCoordinates                    # Get sibling coordinates         [sibling_coordinate_word, node_pointer, ...]
            exec.getChildrenFromCoordinates        # Get Left/Right nephew pointers  [Lnephew_pointer, Rnephew_pointer, node_pointer, ...]
            dup.2 exec.isLeftChild                 # Is node left child?             [isLeftChild_bool, Lnephew_pointer, Rnephew_pointer, node_pointer, ...]
            if.true
              #
              # Comment: Right nephew is the distant nephew
              #
              swap                                 # Distant nephew to top of stack  [distantNephew_pointer, closeNephew_pointer, node_pointer, ...]  
            end 
            dup exec.isNIL not swap                # Is distant nephew not NIL       [distantNephew_pointer, isDistantNephewNotNIL_bool, closeNephew_pointer, node_pointer, ...]  
            exec.getColor                          # Is distant nephew RED           [isDistantNephewRED_bool, isDistantNephewNotNIL_bool, closeNephew_pointer, node_pointer, ...]
            and                                    # Is distant nephew RED & notNIL  [isDistantNephewREDandNotNIL_bool, closeNephew_pointer, node_pointer, ...]
            if.true 
              drop exec.DeleteNode_CaseD6          # Go to delete case D6            [node_pointer, ...]
            else 

              dup exec.isNIL not swap                # Is close nephew not NIL       [closeNephew_pointer, isCloseNephewNotNIL_bool, node_pointer, ...]  
              exec.getColor                          # Is close nephew RED           [isCloseNephewRED_bool, isCloseNephewNotNIL_bool, node_pointer, ...]
              and                                    # Is close nephew RED & notNIL  [isCloseNephewREDandNotNIL_bool, node_pointer, ...]
              if.true 
                exec.DeleteNode_CaseD5               # Go to delete case D5          [node_pointer, ...]
              else   
                exec.DeleteNode_CaseD4               # Go to case D4                 [node_pointer, ...]
              end 
            end 
          end

          dup exec.setToNIL              # Zero out node                           [node_pointer, ...]
          push.0 swap                    # Set recursion flag                      [node_pointer, 0, ...]
        end
      end
    end
  #### END COMPLEX CASES ####
  end
end

proc.DestroyNode
  # Inputs:  [node_pointer, ...]
  # Outputs: [...]
  #
  # Comments: Destroys node whose pointer appears at the top of stack
  #           by removing it from tree
  #
  push.1                                 # Enter while loop                        [1, node_pointer, ...]
  while.true
    exec.DeleteNode                      # Enter DeleteNode subroutine for cases   [node_pointer, recursion_flag, ...] 
    swap                                 # Bring recursion flag top of stack       [recursion_flag, node_pointer, ...]
    ### ITERATE IF RECURSION FLAG IS TRUE ###
  end
  drop                                
end

proc.NewCreation
  # Inputs:  [...]
  # Outputs: [NewNode_pointer, creation_number++...]
  #
  #          Comments: increments creation number and return fresh node pointer
  #
  #### CHECK IF TREE NEEDS TO BE INITIALIZED  ####
  mem_load.0           # Load current creation number state                  [creation_number ...]
  dup push.0 eq        # Check if this is the first node of the tree         [isFirstNode_bool, creation_number ...]

  if.true
    exec.InitTree      # Initialize Tree                                     [creation_number, order_word, ...]
  end

  #### GET CREATION NUMBER FOR NEW NODE ####
  push.1 add               # Increment creation number                       [creation_number++, ...]
  dup mem_store.0          # Save New Creation number                        [creation_number++, ...]
  dup exec.getNodePointer  # Derive node pointer                             [newNode_pointer, creation_number++, ...]
end

proc.CreateNode
  # Inputs:  [order_wordA, ...]
  # Outputs: [...]
  #            Coordinates> {{color, Parent_pointer, L_child_pointer, R_child_pointer}} saved as word 
  #            at fresh node_pointer. Node_pointer computed by using getNodePointer.
  #            Order Info>  {{order_word}} saved at node_pointer+1
  #            [creation_number] -> [creation_number+1] in mem_store.0
  #

  exec.NewCreation                     # Fresh pointer and creation number       [node_pointer, creation_number++, order_word, ...]
  loc_store.1                          # Save node_pointer in registry           [creation_number++, order_word, ...]

  ### GET PARENT ###
  push.1 eq                            # Is this the first node of the tree      [isFirstNode_bool, order_word, ...]
  if.true
    push.0 exec.getNodePointer         # Get tree init addres                    [tree_init_pointer, order_word]
  else
    exec.FindParent                    # Find suitable parent in tree            [parent_pointer, order_word]
  end
  dup loc_store.2                      # Save Parent_pointer in registry         [parent_pointer, order_word, ...]

  #### BUILD NODE COORDINATES ####
  push.0.0                             # Initialize Left and Right child pointers to NIL value      [0, 0, parent_pointer, order_word, ...]
  movup.2                              # Bring parent pointer top of stack                          [parent_pointer, 0, 0, order_word, ...]
  push.1                               # Set default new leaf color to RED                          [1, parent_pointer, 0, 0, order_word, ...]

  ### SAVE COORDINATES AND ORDER WORDS ###
  loc_load.1 exec.UpdateNode             # Save new node                                              [node_pointer, ...]

  ### UPDATE PARENT-CHILD RELATIONSHIP ###
  loc_load.1 loc_load.2                # Load node & parent pointers                                [parent_pointer, node_pointer, ...]
  exec.UpdateChildInParent             # Update child coordinate in parent                          [...]

  #### BALANCE TREE ####
  loc_load.1 exec.BalanceTree          # Execute tree rebalancing operations                        [...]
end

proc.getRightMaximumSubtreeElement
  # Inputs: [node_pointer,....]
  # Ouputs: [Max_Right_subtree_pointer, ...]
  #
  # Comments: Starting from the node N, function traverses right subtree 
  #           to find max subtree node Y.
  #
  dup push.1                        # Duplicate pointer and enter loop       [1, node_pointer, node_pointer, ...]
  while.true
    swap drop                       # Drop node_pointer                      [node_pointer, ...]
    dup exec.getCoordinates         # Get node coordinates                   [node_coordinate_word, node_pointer, ...]
    exec.getChildrenFromCoordinates # Get child pointer                      [Lchild_pointer, Rchild_pointer, node_pointer, ...]
    drop                            # Isolate Right child                    [Rchild_pointer, node_pointer, ...]
    dup push.0 neq                  # Does right child exist?                [doesRchildExist_bool, Rchild_pointer, node_pointer, ...]
  end
  drop
end

proc.getLeftMinimumSubtreeElement
  # Inputs: [node_pointer,....]
  # Ouputs: [Max_Right_subtree_pointer, ...]
  #
  # Comments: Starting from the node N, function traverses right subtree 
  #           to find max subtree node Y.
  #
  dup push.1                        # Duplicate pointer and enter loop       [1, node_pointer, node_pointer, ...]
  while.true
    swap drop                       # Drop node_pointer                      [node_pointer, ...]
    dup exec.getCoordinates         # Get node coordinates                   [node_coordinate_word, node_pointer, ...]
    exec.getChildrenFromCoordinates # Get child pointer                      [Lchild_pointer, Rchild_pointer, node_pointer, ...]
    swap drop                       # Isolate Left child                     [Lchild_pointer, node_pointer, ...]
    dup push.0 neq                  # Does right child exist?                [doesLchildExist_bool, Lchild_pointer, node_pointer, ...]
  end
  drop
end

proc.FillOrder
  # Inputs:  [ask_pointer, bid_quantity, bid_price...]
  # Outputs: [continue_bool, ask_pointer, remaining_bid_quantity, bid_price...]
  #
  # Comments: Fills order at order pointer.           
  #
  dup exec.getOrder      # Load order                      [ask_quantity, ask_price, _____, ____, ask_pointer, bid_quantity, bid_price...]
  dup.1 dup.7 gt         # Is ask price > bid price?       [isAskGreaterThanBid_bool, ask_quantity, ask_price, _____, ____, ask_pointer, bid_quantity, bid_price, ...]
  if.true  
    dup dup.6            # Duplicate quantities on top     [bid_quantity, ask_quantity, ask_quantity, ask_price, _____, ____, ask_pointer, bid_quantity, bid_price, ...]
    dup.1 dup.1 lte      # Is ask quantity < bid quantity? [isAskLessThanBid, bid_quantity, ask_quantity, ask_quantity, ask_price, _____, ____, ask_pointer, bid_quantity, bid_price, ...] 
    if.true 
      #
      # Ask is less than the bid. Execute entire ask order and continue
      #
      swap sub           # How much has been filled      [(bid_quantity-ask_quantity), ask_quantity, ask_price, _____, ____, ask_pointer, bid_quantity, bid_price, ...]
      swap.6 drop        # Update how much left to fill  [ask_quantity, ask_price, _____, ____, ask_pointer, (bid_quantity-ask_quantity), bid_price, ...]
      dropw              # Drop order word               [ask_pointer, (bid_quantity-ask_quantity), bid_price, ...]
      push.1             # Continue filling              [1, ask_pointer, (bid_quantity-ask_quantity), bid_price, ...]       
    else 
      #
      # Ask is greater than the bid. Execute at this price and finish
      #
      sub swap drop              # Update to ask quantity        [(ask_quantity-bid_quantity), ask_price, _____, ____, ask_pointer, bid_quantity, bid_price, ...]
      dup.4 exec.UpdateOrder  # Update order word             [ask_pointer, bid_quantity, bid_price, ...]
      push.0 swap.2 drop      # Update remaining amount       [ask_pointer, 0, bid_price, ...]
      push.0                  # Stop filling                  [0, ask_pointer, 0, bid_price, ...]
    end
  else 
     dropw push.0            # No order to fill              [0, ask_pointer, bid_quantity, bid_price, ...]
  end
end

proc.MatchOrder
  # Inputs:  [order_word, ...]
  # Outputs: [...]
  #
  # Comments: Matches order. Order Flag must be 0 for sell order, 1 for buy order            
  #

  ### SAVE Order Temporarily at proper pointer ####
  dupw                                                      # Duplicate order word                [order_word, order_word, ...]
  exec.NewCreation                                          # Fresh pointer and creation number   [node_pointer, creation_number++, order_word, order_word, ...]
  swap mem_store.0                                          # Save creation number                [node_pointer, order_word, order_word, ...]
  exec.SaveOrder                                            # Save new order                      [order_word, ...]

  loc_store.0 loc_store.1 drop drop                          # Save quantity (loc0) and price (loc1)  [...]
  loc_load.1 loc_load.0                                      # Quantity and Price on stack [quantity, price, ...] 
  exec.getRootPointer exec.getRightMaximumSubtreeElement     # Load pointer to max price   [maxRight_pointer, quantity, price, ...]
  push.1                                                     # Enter loop                  [1, maxRight_pointer, quantity, price, ...]
  while.true 
    exec.FillOrder                                           # Fill order                  [continuation_bool, maxRight_pointer, remaining_quantity, price, ...]
    if.true 
      dup exec.getCoordinates                                # Get ask order coordiantes   [___, ask_parent_pointer, ask_Lchild_pointer, ___, maxRight_pointer, quantity, price, ...]
      drop swap dup push.0 eq                                # Is Left child NIL           [isLeftChildNIL_bool, ask_Lchild_pointer, ask_parent_pointer, ___, maxRight_pointer, quantity, price, ...]
      if.true 
        drop swap drop swap                                  # Isolate parent              [maxRight_pointer, ask_parent_pointer, quantity, price, ...]  
      else 
        swap drop swap drop swap                             # Isolate Left child          [maxRight_pointer, ask_Lchild_pointer, quantity, price, ...]  
      end
      exec.DestroyNode                                       # Destroy Filled order        [next_ask_pointer, quantity, price, ...]
      push.1                                                 # Continue filling            [1, next_ask_pointer, quantity, price, ...]
    else 
     drop drop drop push.0                                   # Finished filling            [0, ...]
     mem_load.0 exec.getNodePointer exec.ZeroOut         # Destroy Bid Order           [0, ...]
     push.0
    end
  end
end

proc.TempLoadOrders
  # Inputs:  [...]
  # Outputs: [...]
  #
  #    Comments: Loads orders and creates rbBST 
  #
  push.0.0.0.0
  adv.push_mapval adv_loadw           # Load Total number of orders in advice stack    [number_of_orders, ...]
  loc_store.0                         # Save number of orders in registry position 0   [...]
  push.0 dup                          # Initialize counter i                           [0, 0, ...]
  loc_load.1 loc_load.0 lt 
  while.true
    push.1 add                        # Increment counter and duplicate              [order_counter++, ...]
    push.0.0.0 dup.3                  # Add buffer for loading next word             [0, 0, 0, 0, order_counter++, ...]
    adv.push_mapval adv_loadw         # Load new order                               [ID, time, price, quantity, order_counter++, ...]
    movdn.3 movdn.2 swap              # Put into correct format                      [quantity, price, time, ID, order_counter++, ...]
    exec.CreateNode                   # Create Node                                  [order_counter++, ...]
    dup loc_load.0 lt                 # Aare there more nodes?                       [more_nodes_bool, order_counter++, ...]
  end
  drop
end

proc.LoadTree
  # Inputs:  [...]
  # Outputs: [...]
  #
  #    Comments: Loads pre-structured tree nodes from advice stack            
  #              Coordinate Word Format: {{color, Parent_pointer, L_child_pointer, R_child_pointer}} 
  #              Order Word Format:      {{quantity, price, time, ID}} 
  #
  push.0.0.0.0
  adv.push_mapval adv_loadw            # Load Total number of orders in advice stack    [number_of_orders, ...]
  mem_store.0 drop drop drop           # Save number of orders as final creation number [...]
  exec.InitTree                       # Initialize Tree                                [...]
  push.0 dup                          # Initialize counter in registry position 1      [0, 0, ...]
  loc_load.1 mem_load.0 lt 
  while.true
    push.1 add                        # Increment counter                            [order_counter++, ...] 
    push.0.0.0 dup.3                  # Add buffer for loading next word             [0, 0, 0, 0, order_counter++, ...]
    adv.push_mapval adv_loadw         # Load Node pointer                            [node_pointer, 0, 0, 0, order_counter++, ...]
    push.0.0.0.0 adv_loadw            # Load Node coordinates                        [Rchild_pointer, Lchild_pointer, Parent_pointer, color, node_pointer, 0, 0, 0, order_counter++, ...]
    movdn.3 movdn.2 swap              # Put into correct format                      [node_coordinate_word, node_pointer, 0, 0, 0, order_counter++, ...]
    push.0.0.0.0 adv_loadw            # Load Node order                              [ID, time, price, quantity, node_coordinate_word, node_pointer, 0, 0, 0, order_counter++, ...]
    movdn.3 movdn.2 swap              # Put into correct format                      [node_order_word, node_coordinate_word, node_pointer, 0, 0, 0, order_counter++, ...]
    swapw                             # Coordinates in front                         [node_coordinate_word, node_order_word, node_pointer, 0, 0, 0, order_counter++, ...]
    movupw.2 movdn.7 drop drop drop   # Isolate node pointer                         [node_coordinate_word, node_pointer, node_order_word, order_counter++, ...]
    dup.4                             # Copy node pointer top of stack               [node_pointer, node_coordinate_word, node_pointer, node_order_word, order_counter++, ...]
    exec.SaveCoordinate               # Save Node coordiante word                    [node_pointer, node_order_word, order_counter++, ...]
    exec.SaveOrder                    # Save Node order word                         [order_counter++, ...]
    dup mem_load.0 lt                 # Aare there more nodes?                      [more_nodes_bool, order_counter++, ...]            
  end
  drop drop
end

begin
  # Operand Stack format: [orderID, time, price, quantity, buy-sell flag]
  # Advice stack format: 
  #            0000000000000000000000000000000000000000000000000000000000000000 : [0,0,0, number_of_orders_in_book],
  #            0000000000000000000000000000000000000000000000000100000000000000 : [0, 0, 0, node_pointer,  color,  parent_pointer, Lchild_pointer, Rchild_pointer, quantity, price, time, ID], 
  #
  # Comments: Buy-sell_flag should be '0' for a sell order, '1' for buy order. Orderbook is assumed to be the BUY side. 
  #           All buy orders will be immediately inserted.
  #
  # ORDER INSERTION EXAMPLE
  #
  # Load pre-structured tree from advice_stack
  exec.LoadTree

  #### Check buy-sell flag
  if.true
    # Order is but order
    # Create new node in tree by inserting new order from operand stack 
    exec.CreateNode  
  else 
    # Order is a sell order
    # Search tree for matching orders
    # and execute 
    exec.MatchOrder
  end

  # Output all the nodes that have to be updated as a result of the new order inserted
  exec.PrintChangeLog

  ###### FOR BUILDING DUMMY STACK ####
  #exec.TempLoadOrders
  #exec.PrintAllNodes #Essential

    ###### FOR BUILDING PLOTTING DUMP ####
  #exec.TempLoadOrders
  #exec.PrintAllNodesEssential

end
")}}
