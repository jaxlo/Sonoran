# jinix
An academic operating system in rust for my IS capstone


## Design choices

Overall
- Build the kernel

Hybrid kernel (A mix of monolithic and micro kernel design)
- Drivers are compiled with the kernel (Like a monolithic kernel)
- Message passing seems like an interesting challenge. Look at L4 for optimized message passing
- Processes 


Memory Management
- The Ox64 has the Sv39 MMU
- Having virtual memory addresses is good for isolating processes logically





Config Files
- Put into a directory for config? like /etc/ but dedicated 
- Store in yaml?
- Build an api like the windows registry in future iterations?


Shell
- Like bash/fish/python | Use tabs for indentation? :P
- Use a fewer set of characters because it would be nice to have a small keyboard.
- Include the most common characters from human langs, programming langs, 


Hardware
- Build a split keyboard and have a vertical screen in the middle?
- Maybe around the size of a smaller smartphone
- One PCB and no battery to keep the design minimal/lean to start?


## Resources used:
- https://os.phil-opp.com/ <- The freestanding binary and kernel start