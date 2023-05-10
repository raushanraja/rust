### IF :

    - syntx: if(condition){
    	statement;
    	}

### IF-ELSE

    - syntax:
    if(condition)
    {
    	statement;
    }
    else
    {
    	statement;
    }

### IF-ELSEIF-ELSE

    -syntax:
        if(condition)
        {
            statement;
        }
        else if(condition2)
        {
        statement;
        }
        else
        {
            statement;
        }

### Loop

    -syntax:
        loop
        {
            statement;
            if(condition)  // Otherwise the loop will be infinite
            {
                break;
            }
        }
    - loop also provides loop label
    - Loop label are used to add a name to the loop that allows to point to speicific loop 
    - This are useful when using nested loop and we want to stop a specific loop, without stopping all
    - syntax:
        `loopLabel: loop  // backtick before loop label
        {
            statement;
            if(condition)  // Otherwise the loop will be infinite
            {
                break `loopLabel;
            }
        }


### For Loop

    - syntax:

        for var in expression
        {
            statement;
        }

    // expression for array with name a `a.iter()`
    // range expession: a..b: eg: for i in 1..10

### While loop

    - syntax:
    	while(condition)
    	{
    		statement;
    	}
