<script>
    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Button } from 'flowbite-svelte';

  </script>

  <div class="bg-black-600">
    <Table hoverable={true} noborder={false} color="bg-black-600">
        <TableHead>
          <TableHeadCell>Parameter</TableHeadCell>
          <TableHeadCell>Limitation</TableHeadCell>
          <TableHeadCell>Value</TableHeadCell>
          <TableHeadCell>
            <span class="sr-only">Edit</span>
          </TableHeadCell>
        </TableHead>
        <TableBody class="divide-y">
          <TableBodyRow>
            <TableBodyCell>temperature</TableBodyCell>
            <TableBodyCell>>=5</TableBodyCell>
            <TableBodyCell>20</TableBodyCell>
            <TableBodyCell>
              <a href="/tables" class="font-medium text-primary-600 hover:underline dark:text-primary-500">Edit</a>
            </TableBodyCell>
          </TableBodyRow>
          <TableBodyRow>
            <TableBodyCell>Microsoft Surface Pro</TableBodyCell>
            <TableBodyCell>White</TableBodyCell>
            <TableBodyCell>Laptop PC</TableBodyCell>
            <TableBodyCell>
              <a href="/tables" class="font-medium text-primary-600 hover:underline dark:text-primary-500">Edit</a>
            </TableBodyCell>
          </TableBodyRow>
          <TableBodyRow>
            <TableBodyCell>Magic Mouse 2</TableBodyCell>
            <TableBodyCell>Black</TableBodyCell>
            <TableBodyCell>Accessories</TableBodyCell>
            <TableBodyCell>
              <a href="/tables" class="font-medium text-primary-600 hover:underline dark:text-primary-500">Edit</a>
            </TableBodyCell>
          </TableBodyRow>
        </TableBody>
      </Table>
  </div>

   