<template>
  <div class="todo-container">
    <el-container>
      <el-header>Todo List</el-header>
      <el-main>
        <div style="margin-bottom: 15px; width: 500px">
          <el-input placeholder="Please enter content" v-model="text">
            <template #append>
              <el-button :icon="Plus" @click="onAddTodo" />
            </template>
          </el-input>
        </div>
        <el-table :data="todos" style="width: 100%" table-layout="fixed" stripe>
          <el-table-column type="index" label="No."></el-table-column>
          <el-table-column prop="description" label="description">
          </el-table-column>
          <el-table-column prop="completed" label="completed">
            <template #default="scope">
              <el-tag
                :type="scope.row.completed === 0 ? 'warning' : 'success'"
                disable-transitions
                >{{ tags[scope.row.completed] }}</el-tag
              >
            </template>
          </el-table-column>
          <el-table-column fixed="right" label="action">
            <template #default="scope">
              <el-button
                @click="onComplete(scope.row)"
                type="primary"
                size="small"
                link
                v-if="scope.row.completed == 0"
              >
                complete
              </el-button>
              <el-button
                @click="onDelete(scope.row)"
                type="primary"
                size="small"
                link
              >
                remove
              </el-button>
            </template>
          </el-table-column>
        </el-table>
        <el-pagination
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
          :current-page="currentPage"
          :page-size="pageSize"
          layout="prev, pager, next"
          :total="total"
        >
        </el-pagination>
      </el-main>
    </el-container>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Plus } from '@element-plus/icons-vue'

interface Todo {
  id: number
  description: string
  completed: number
}
const currentPage = ref<number>(1)
const pageSize = ref<number>(10)
const total = ref<number>(0)
const todos = ref<Todo[]>([])
const text = ref('')
const todoUrl = 'http://127.0.0.1:3000/todos'
const tags = ['未完成', '已完成']
const fetchTodos = async (page: number, size: number) => {
  try {
    const response = await fetch(
      todoUrl + '?page=' + page + '&page_size=' + size
    )
    if (response.ok) {
      let resp = await response.json()
      if (resp.code == 0) {
        todos.value = await resp.data.records
        total.value = resp.data.total
      }
    }
  } catch (error) {
    console.error('Fetch error:', error)
  }
}
onMounted(() => {
  fetchTodos(currentPage.value, pageSize.value)
})

const onAddTodo = async () => {
  if (text.value.trim() !== '') {
    const data = { description: text.value.trim() }
    try {
      const response = await fetch(todoUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
      })
      if (response.ok) {
        let resp = await response.json()
        if (resp.code == 0) {
          fetchTodos(currentPage.value, pageSize.value)
        }
      }
    } catch (error) {
      console.error('Fetch error:', error)
    }
    text.value = ''
  }
}

const onComplete = async (todo: Todo) => {
  const data = { id: todo.id, description: todo.description, completed: 1 }
  try {
    const response = await fetch(todoUrl, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    })
    if (response.ok) {
      let resp = await response.json()
      if (resp.code == 0) {
        fetchTodos(currentPage.value, pageSize.value)
      }
    }
  } catch (error) {
    console.error('Fetch error:', error)
  }
}

const onDelete = async (todo: Todo) => {
  try {
    const response = await fetch(todoUrl + '/' + todo.id, {
      method: 'DELETE'
    })
    if (response.ok) {
      let resp = await response.json()
      if (resp.code == 0) {
        fetchTodos(currentPage.value, pageSize.value)
      }
    }
  } catch (error) {
    console.error('Fetch error:', error)
  }
}

const handleSizeChange = (val: number) => {
  pageSize.value = val
  fetchTodos(currentPage.value, pageSize.value)
}

const handleCurrentChange = (val: number) => {
  currentPage.value = val
  fetchTodos(currentPage.value, pageSize.value)
}
</script>
